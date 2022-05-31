import init, * as wasm from "./wasm/fluid_wasm.js";

const SIZE = 150;
const SPEED = 0.005;
const DIFFUSION = 0.;
const VISCOSITY = 0.0005;

let clientWidth = document.documentElement.clientWidth;
let clientHeight = document.documentElement.clientHeight;
let minimum = Math.min(clientWidth, clientHeight);
let SCALE = minimum / SIZE * 2;

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
            // let x = Math.round(e.offsetX / SCALE);
            let x = Math.round((e.offsetX / document.documentElement.clientWidth) * SIZE);
            let y = Math.round((e.offsetY / document.documentElement.clientHeight) * SIZE);
            simulation.add_density(x, y, 5.);
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
blur = function() {

    var ctx = canvas.getContext("2d");
    ctx.globalAlpha = 0.4;

    var offset = 3;

    for (var i = 1; i <= 8; i += 1) {
        ctx.drawImage(canvas, offset, 0, canvas.width - offset, canvas.height, 0, 0, canvas.width - offset, canvas.height);
        ctx.drawImage(canvas, 0, offset, canvas.width, canvas.height - offset, 0, 0, canvas.width, canvas.height - offset);
    }
};

function mainloop(simulaiton) {
    simulaiton.step();
    simulaiton.draw_density(SCALE);
    blur()
    // simulaiton.draw_velocity();
    window.requestAnimationFrame(() => {
        mainloop(simulaiton)
    })
}

run().catch(console.error);

