mod simulation;
use sdl2::{
    event::Event, keyboard::Keycode, libc::SIGEMT, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};
use simulation::Simulation;

// Constants
const WINDOW_SIZE: u32 = 800;
const NUM_CELLS: u32 = 200;
const CELL_WIDTH: u32 = WINDOW_SIZE / NUM_CELLS;

pub fn run() {
    // Simulation Setup
    let mut simulation = Simulation::new(0.005, 0.0005, 0.0, NUM_CELLS as usize);

    //SDL setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Fluid Simulation", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let colour = colorgrad::inferno();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut past_mouse_x: i32 = 0;
    let mut past_mouse_y: i32 = 0;

    let mut display_velocity = false;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => display_velocity = !display_velocity,
                _ => (),
            }
        }

        if event_pump.mouse_state().left() {
            let x = (event_pump.mouse_state().x() as u32 / CELL_WIDTH)
                .max(1)
                .min(NUM_CELLS - 2);
            let y = (event_pump.mouse_state().y() as u32 / CELL_WIDTH)
                .max(1)
                .min(NUM_CELLS - 2);

            simulation.add_density(x, y, 1.);
            simulation.add_density(x + 1, y, 1.);
            simulation.add_density(x - 1, y, 1.);
            simulation.add_density(x, y + 1, 1.);
            simulation.add_density(x, y - 1, 1.);
        }

        if event_pump.mouse_state().right() {
            let x = event_pump.mouse_state().x();
            let y = event_pump.mouse_state().y();
            let dx = x - past_mouse_x;
            let dy = y - past_mouse_y;
            simulation.add_velocity(
                (x as u32 / CELL_WIDTH).max(1).min(NUM_CELLS - 2),
                (y as u32 / CELL_WIDTH).max(1).min(NUM_CELLS - 2),
                dx as f32 * 5.,
                dy as f32 * 5.,
            );
            past_mouse_x = x;
            past_mouse_y = y;
        } else {
            past_mouse_x = event_pump.mouse_state().x();
            past_mouse_y = event_pump.mouse_state().y();
        }
        simulation.step();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_density(&mut canvas, &simulation, &colour);
        if display_velocity {
            draw_velocity_arrows(&mut canvas, &simulation)
        }
        canvas.present();
    }
}
fn draw_density(
    canvas: &mut Canvas<Window>,
    simulation: &Simulation,
    colour: &colorgrad::Gradient,
) {
    let density = simulation.density();

    for x in 0..NUM_CELLS {
        for y in 0..NUM_CELLS {
            let d = density[simulation.ix(x, y) as usize];
            let (r, g, b, _) = colour.at(d.min(1.) as f64).rgba_u8();
            canvas.set_draw_color(Color::RGB(r, g, b));

            let rect = Rect::new(
                (x * CELL_WIDTH) as i32,
                (y * CELL_WIDTH) as i32,
                CELL_WIDTH,
                CELL_WIDTH,
            );

            canvas.fill_rect(rect).unwrap();
        }
    }
}

fn draw_velocity_arrows(canvas: &mut Canvas<Window>, simulation: &Simulation) {
    let vel_x = simulation.vel_x();
    let vel_y = simulation.vel_y();

    canvas.set_draw_color(Color::RGB(255, 0, 0));

    for x in 0..NUM_CELLS {
        for y in 0..NUM_CELLS {
            // Velocity start points
            let start_x = x * CELL_WIDTH + CELL_WIDTH / 2;
            let start_y = y * CELL_WIDTH + CELL_WIDTH / 2;

            // Change in x and y
            let dx = (vel_x[simulation.ix(x, y) as usize] * 1000.).round() / 1000.;
            let dy = (vel_y[simulation.ix(x, y) as usize] * 1000.).round() / 1000.;

            // Set to 1 if velocity is 0
            let normalising_factor = if dx == 0. && dy == 0. {
                1.
            } else {
                CELL_WIDTH as f32 / (dx * dx + dy * dy).sqrt()
                // 1.
            };

            // Velocity end points
            let end_x = start_x as f32 + dx * normalising_factor;
            let end_y = start_y as f32 + dy * normalising_factor;

            // Draw velocity line
            canvas
                .draw_line(
                    (start_x as i32, start_y as i32),
                    (end_x as i32, end_y as i32),
                )
                .unwrap();
        }
    }
}
