import {App} from "special-relativity-web";

const canvas = document.getElementById("canvas") as HTMLCanvasElement | null;
if (!canvas) {
    throw new Error("No 'canvas'");
}

const screenWidth = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
const screenHeight = window.innerHeight || document.documentElement.clientHeight || document.body.clientHeight;
if (screenWidth < screenHeight) {
    canvas.width = screenWidth;
    canvas.height = screenHeight * 0.8;
} else {
    canvas.width = 1200;
    canvas.height = 800;
}

const context = canvas.getContext("webgl2");
if (!context) {
    throw new Error("webgl2 not supported");
}
context.viewport(0, 0, canvas.width, canvas.height);

const info = document.getElementById("info")!;

const app = new App(context);

document.addEventListener('keydown', (event) => {
    app.key_down(event.key.toLowerCase());
});
document.addEventListener('keyup', (event) => {
    app.key_up(event.key.toLowerCase());
});
window.addEventListener('blur', () => {
    app.window_blue();
});

canvas.addEventListener('touchstart', (event) => {
    event.preventDefault();
    const x = [];
    const y = [];
    for (let i = 0; i < event.touches.length; i++) {
        x.push(event.touches[i].clientX);
        y.push(event.touches[i].clientY);
    }
    app.touch_start(new Float64Array(x), new Float64Array(y));
});
canvas.addEventListener('touchmove', (event) => {
    event.preventDefault();
    const x = [];
    const y = [];
    for (let i = 0; i < event.touches.length; i++) {
        x.push(event.touches[i].clientX);
        y.push(event.touches[i].clientY);
    }
    app.touch_move(new Float64Array(x), new Float64Array(y));
});
canvas.addEventListener('touchend', () => {
    app.touch_end();
});

const buttonUp = document.getElementById("button-up")!;
const buttonDown = document.getElementById("button-down")!;
const buttonLeft = document.getElementById("button-left")!;
const buttonRight = document.getElementById("button-right")!;
buttonUp.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowup");
});
buttonUp.addEventListener("touchend", (event) => {
    app.key_up("arrowup");
});
buttonDown.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowdown");
});
buttonDown.addEventListener("touchend", (event) => {
    app.key_up("arrowdown");
});
buttonLeft.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowleft");
});
buttonLeft.addEventListener("touchend", (event) => {
    app.key_up("arrowleft");
});
buttonRight.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowright");
});
buttonRight.addEventListener("touchend", (event) => {
    app.key_up("arrowright");
});

const presetNodes = document.getElementsByName("preset") as NodeListOf<HTMLInputElement>;

const presetChange = () => {
    for (let i = 0; i < presetNodes.length; i++) {
        if (presetNodes.item(i).checked) {
            console.log(presetNodes.item(i).value);
            app.reset_charge(presetNodes.item(i).value);
            break;
        }
    }
};
const preset1 = document.getElementById("preset1") as HTMLSelectElement;
preset1.onchange = presetChange;
const preset2 = document.getElementById("preset2") as HTMLSelectElement;
preset2.onchange = presetChange;
const preset3 = document.getElementById("preset3") as HTMLSelectElement;
preset3.onchange = presetChange;


function step(timestamp: DOMHighResTimeStamp): void {
    app.tick(timestamp);
    info.innerText = app.info();
    window.requestAnimationFrame(step);
}

window.requestAnimationFrame(step);
