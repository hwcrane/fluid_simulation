use std::mem::swap;

pub struct Simulation {
    speed: f32,
    viscosity: f32,
    diffusion: f32,
    dispersion: f32,

    // number of cells vertically and horesontally
    size: u32,

    // Vecs storing the density
    density: Vec<f32>,
    prev_density: Vec<f32>,

    // Vecs storing the horisontal and virtical velocity
    vel_x: Vec<f32>,
    vel_y: Vec<f32>,
    prev_vel_x: Vec<f32>,
    prev_vel_y: Vec<f32>,
}

impl Simulation {
    pub fn new(speed: f32, viscosity: f32, diffusion: f32, size: usize) -> Simulation {
        Simulation {
            speed,
            viscosity,
            diffusion,
            dispersion: 0.0005,
            size: size as u32,
            density: vec![0.; size * size],
            prev_density: vec![0.; size * size],
            vel_x: vec![0.; size * size],
            vel_y: vec![0.; size * size],
            prev_vel_x: vec![0.; size * size],
            prev_vel_y: vec![0.; size * size],
        }
    }

    pub fn vel_x(&self) -> &Vec<f32> {
        &self.vel_x
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn vel_y(&self) -> &Vec<f32> {
        &self.vel_y
    }

    pub fn density(&self) -> &Vec<f32> {
        &self.density
    }

    pub fn ix(&self, x: u32, y: u32) -> usize {
        (x + y * self.size) as usize
    }

    pub fn add_density(&mut self, x: u32, y: u32, amount: f32) {
        let index = self.ix(x, y) as usize;
        self.density[index] += amount;
    }

    pub fn add_velocity(&mut self, x: u32, y: u32, vx: f32, vy: f32) {
        let index = self.ix(x, y) as usize;
        self.vel_x[index] += vx;
        self.vel_y[index] += vy;
    }

    pub fn step(&mut self) {
        self.density_step();
        self.velocity_step();
    }

    pub fn velocity_step(&mut self) {
        swap(&mut self.vel_x, &mut self.prev_vel_x);
        swap(&mut self.vel_y, &mut self.prev_vel_y);
        diffuse(
            self.size,
            1,
            &mut self.vel_x,
            &self.prev_vel_x,
            self.viscosity,
            self.speed,
        );
        diffuse(
            self.size,
            2,
            &mut self.vel_y,
            &self.prev_vel_y,
            self.viscosity,
            self.speed,
        );
        project(
            self.size,
            &mut self.vel_x,
            &mut self.vel_y,
            &mut self.prev_vel_x,
            &mut self.prev_vel_y,
        );
        swap(&mut self.vel_x, &mut self.prev_vel_x);
        swap(&mut self.vel_y, &mut self.prev_vel_y);

        advect(
            self.size,
            1,
            &mut self.vel_x,
            &self.prev_vel_x,
            &self.prev_vel_x,
            &self.prev_vel_y,
            self.speed,
        );
        advect(
            self.size,
            2,
            &mut self.vel_y,
            &self.prev_vel_y,
            &self.prev_vel_x,
            &self.prev_vel_y,
            self.speed,
        );
        project(
            self.size,
            &mut self.vel_x,
            &mut self.vel_y,
            &mut self.prev_vel_x,
            &mut self.prev_vel_y,
        );
    }

    pub fn density_step(&mut self) {
        swap(&mut self.density, &mut self.prev_density);
        diffuse(
            self.size,
            0,
            &mut self.density,
            &self.prev_density,
            self.diffusion,
            self.speed,
        );
        swap(&mut self.density, &mut self.prev_density);
        advect(
            self.size,
            0,
            &mut self.density,
            &self.prev_density,
            &self.vel_x,
            &self.vel_y,
            0.005,
        );
        disperse(self.dispersion, &mut self.density);
    }
}

fn ix(x: u32, y: u32, size: u32) -> usize {
    (x + y * size) as usize
}

fn project(
    size: u32,
    vel_x: &mut Vec<f32>,
    vel_y: &mut Vec<f32>,
    prev_vel_x: &mut Vec<f32>,
    prev_vel_y: &mut Vec<f32>,
) {
    let h = 1. / (size - 2) as f32;

    for i in 1..=(size - 2) {
        for j in 1..=(size - 2) {
            prev_vel_y[ix(i, j, size)] = -0.5
                * h
                * (vel_x[ix(i + 1, j, size)] - vel_x[ix(i - 1, j, size)]
                    + vel_y[ix(i, j + 1, size)]
                    - vel_y[ix(i, j - 1, size)]);
            prev_vel_x[ix(i, j, size)] = 0.;
        }
    }

    set_bnd(size, 0, prev_vel_y);
    set_bnd(size, 0, prev_vel_x);

    for _ in 0..20 {
        for i in 1..=(size - 2) {
            for j in 1..=(size - 2) {
                prev_vel_x[ix(i, j, size)] = (prev_vel_y[ix(i, j, size)]
                    + prev_vel_x[ix(i - 1, j, size)]
                    + prev_vel_x[ix(i + 1, j, size)]
                    + prev_vel_x[ix(i, j - 1, size)]
                    + prev_vel_x[ix(i, j + 1, size)])
                    / 4.;
            }
        }
        set_bnd(size, 0, prev_vel_x);
    }

    for i in 1..=(size - 2) {
        for j in 1..=(size - 2) {
            vel_x[ix(i, j, size)] -=
                0.5 * (prev_vel_x[ix(i + 1, j, size)] - prev_vel_x[ix(i - 1, j, size)]) / h;

            vel_y[ix(i, j, size)] -=
                0.5 * (prev_vel_x[ix(i, j + 1, size)] - prev_vel_x[ix(i, j - 1, size)]) / h;
        }
    }

    set_bnd(size, 1, vel_x);
    set_bnd(size, 2, vel_y);
}
fn diffuse(
    size: u32,
    b: u32,
    current: &mut Vec<f32>,
    prev: &Vec<f32>,
    diffuse_rate: f32,
    speed: f32,
) {
    let a = speed * diffuse_rate * ((size - 2) * (size - 2)) as f32;
    for _ in 0..20 {
        for x in 1..=(size - 2) {
            for y in 1..=(size - 2) {
                current[ix(x, y, size)] = (prev[ix(x, y, size)]
                    + a * (current[ix(x - 1, y, size)]
                        + current[ix(x + 1, y, size)]
                        + current[ix(x, y - 1, size)]
                        + current[ix(x, y + 1, size)]))
                    / (1. + 4. * a)
            }
        }
        set_bnd(size, b, current)
    }
}

fn advect(
    size: u32,
    b: u32,
    current: &mut Vec<f32>,
    prev: &Vec<f32>,
    vel_x: &Vec<f32>,
    vel_y: &Vec<f32>,
    speed: f32,
) {
    let dt0 = speed * (size - 2) as f32;
    for i in 1..=(size - 2) {
        for j in 1..=(size - 2) {
            let mut x = i as f32 - dt0 * vel_x[ix(i, j, size)];
            let mut y = j as f32 - dt0 * vel_y[ix(i, j, size)];

            if x < 0.5 {
                x = 0.5
            };
            if x > size as f32 - 2. + 0.5 {
                x = size as f32 - 2. + 0.5
            };
            let i0 = x as u32;
            let i1 = i0 + 1;

            if y < 0.5 {
                y = 0.5
            };
            if y > size as f32 - 2. + 0.5 {
                y = size as f32 - 2. + 0.5
            };
            let j0 = y as u32;
            let j1 = j0 + 1;

            let s1 = x - i0 as f32;
            let s0 = 1. - s1;

            let t1 = y - j0 as f32;
            let t0 = 1. - t1;

            current[ix(i, j, size)] = s0
                * (t0 * prev[ix(i0, j0, size)] + t1 * prev[ix(i0, j1, size)])
                + s1 * (t0 * prev[ix(i1, j0, size)] + t1 * prev[ix(i1, j1, size)])
        }
    }
    set_bnd(size, b, current)
}

fn disperse(amount: f32, density: &mut Vec<f32>) {
    density.iter_mut().for_each(|x| *x -= amount * *x)
}

fn set_bnd(size: u32, b: u32, current: &mut Vec<f32>) {
    for i in 1..(size - 1) {
        current[ix(i, 0, size)] = if b == 2 {
            -current[ix(i, 1, size)]
        } else {
            current[ix(i, 1, size)]
        };

        current[ix(i, size - 1, size)] = if b == 2 {
            -current[ix(i, size - 2, size)]
        } else {
            current[ix(i, size - 2, size)]
        }
    }

    for i in 1..(size - 1) {
        current[ix(0, i, size)] = if b == 1 {
            -current[ix(1, i, size)]
        } else {
            current[ix(1, i, size)]
        };

        current[ix(size - 1, i, size)] = if b == 1 {
            -current[ix(size - 2, i, size)]
        } else {
            current[ix(size - 2, i, size)]
        }
    }

    current[ix(0, 0, size)] = 0.5 * (current[ix(1, 0, size)] + current[ix(0, 1, size)]);
    current[ix(0, size - 1, size)] =
        0.5 * (current[ix(1, size - 1, size)] + current[ix(0, size - 2, size)]);
    current[ix(size - 1, 0, size)] =
        0.5 * (current[ix(size - 2, 0, size)] + current[ix(size - 1, 1, size)]);
    current[ix(size - 1, size - 1, size)] =
        0.5 * (current[ix(size - 2, size - 1, size)] + current[ix(size - 1, size - 2, size)]);
}
