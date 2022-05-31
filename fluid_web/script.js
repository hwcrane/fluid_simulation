import init, * as wasm from "./wasm/fluid_wasm.js";

const SIZE = 200;
const SCALE = 10;

const canvas = document.getElementById("canvas");
canvas.width = SIZE * SCALE;
canvas.height = SIZE * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, SIZE * SCALE, SIZE * SCALE);

async function run() {
    await init();
    let simulation = new wasm.SimulationWasm();

    canvas.addEventListener('mousemove', e => {
        if (e.buttons == 1) {
            console.log(e);
            let x = Math.round(e.offsetX / SCALE);
            let y = Math.min(Math.round(e.offsetY / SCALE), SIZE - 1);
            simulation.add_density(x, y, 1.);
            simulation.add_velocity(x, y, e.movementX * 5, e.movementY * 5);
        }
    });

    mainloop(simulation);
}

function mainloop(simulaiton) {
    simulaiton.step();
    simulaiton.draw_density(SCALE);
    simulaiton.draw_velocity();
    window.requestAnimationFrame(() => {
        mainloop(simulaiton)
    })
}

run().catch(console.error);

