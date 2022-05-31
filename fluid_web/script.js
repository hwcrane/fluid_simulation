import init, * as wasm from "./wasm/fluid_wasm.js";

const SIZE = 100;
const SPEED = 0.005;
const DIFFUSION = 0.;
const VISCOSITY = 0.0005;

let clientWidth = document.documentElement.clientWidth;
let clientHeight = document.documentElement.clientHeight;
let minimum = Math.min(clientWidth, clientHeight);
let SCALE = minimum / SIZE;

const canvas = document.getElementById("canvas");
canvas.width = SIZE * SCALE;
canvas.height = SIZE * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, SIZE * SCALE, SIZE * SCALE);
canvas.addEventListener("touchstart", function(event) { event.preventDefault() })
canvas.addEventListener("touchmove", function(event) { event.preventDefault() })
canvas.addEventListener("touchend", function(event) { event.preventDefault() })
canvas.addEventListener("touchcancel", function(event) { event.preventDefault() })

let prevTouchX = 0.;
let prevTouchY = 0.;

async function run() {
    await init();
    let simulation = new wasm.SimulationWasm(SPEED, VISCOSITY, DIFFUSION, SIZE);

    canvas.addEventListener('mousemove', e => {
        if (e.buttons == 1) {
            let x = Math.round(e.offsetX / SCALE);
            let y = Math.min(Math.round(e.offsetY / SCALE), SIZE - 1);
            simulation.add_density(x, y, 1.);
            simulation.add_velocity(x, y, e.movementX * 5, e.movementY * 5);
        }
    });

    canvas.addEventListener("touchmove", e => {
        let offsetx = canvas.offsetLeft;
        let offsety = canvas.offsetTop;
        let x = e.changedTouches[0].pageX - offsetx;
        let y = e.changedTouches[0].pageY - offsety;
        let dx = x - prevTouchX;
        let dy = y - prevTouchY;

        let gridX = Math.max(Math.min(x / SCALE, SIZE - 1), 0);
        let gridY = Math.max(Math.min(y / SCALE, SIZE - 1), 0);

        simulation.add_density(gridX, gridY, 1.);
        simulation.add_velocity(gridX, gridY, dx * 5, dy * 5);
        prevTouchX = x;
        prevTouchY = y;
    })

    mainloop(simulation);
}

function mainloop(simulaiton) {
    simulaiton.step();
    simulaiton.draw_density(SCALE);
    // simulaiton.draw_velocity();
    window.requestAnimationFrame(() => {
        mainloop(simulaiton)
    })
}

run().catch(console.error);

