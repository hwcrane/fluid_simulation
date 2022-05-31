use fluid_engine::Simulation;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct SimulationWasm {
    simulation: Simulation,
    ctx: CanvasRenderingContext2d,
    colour: colorgrad::Gradient,
}

#[wasm_bindgen]
impl SimulationWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(speed: f32, viscosity: f32, diffusion: f32, size: usize) -> SimulationWasm {
        let simulation = Simulation::new(speed, viscosity, diffusion, size);
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let colour = colorgrad::inferno();

        SimulationWasm {
            simulation,
            ctx,
            colour,
        }
    }

    #[wasm_bindgen]
    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        self.simulation.add_density(x as u32, y as u32, amount);
    }

    #[wasm_bindgen]
    pub fn add_velocity(&mut self, x: usize, y: usize, dx: f32, dy: f32) {
        self.simulation.add_velocity(x as u32, y as u32, dx, dy)
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        self.simulation.step();
    }

    #[wasm_bindgen]
    pub fn draw_density(&self, scale: f32) {
        let density = self.simulation.density();
        for x in 0..self.simulation.size() {
            for y in 0..self.simulation.size() {
                let index = self.simulation.ix(x, y);
                let d = density[index];
                let (r, g, b, _) = self.colour.at(d as f64).rgba_u8();

                self.ctx
                    .set_fill_style(&JsValue::from_str(&format!("rgb({}, {}, {})", r, g, b)));

                self.ctx.fill_rect(
                    x as f64 * scale as f64,
                    y as f64 * scale as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }

    #[wasm_bindgen]
    pub fn draw_velocity(&self) {
        // todo!()
    }
}
