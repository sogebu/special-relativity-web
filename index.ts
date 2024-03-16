import {App} from "special-relativity-web";

const canvas = document.getElementById("canvas") as HTMLCanvasElement | null;
if (!canvas) {
    throw new Error("No 'canvas'");
}
canvas.width = 1200;
canvas.height = 800;
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

const buttonUp = document.getElementById("button-up")!;
const buttonDown = document.getElementById("button-down")!;
const buttonLeft = document.getElementById("button-left")!;
const buttonRight = document.getElementById("button-right")!;
buttonUp.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowup");
});
buttonUp.addEventListener("touchend", (event) => {
    event.preventDefault();
    app.key_up("arrowup");
});
buttonDown.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowdown");
});
buttonDown.addEventListener("touchend", (event) => {
    event.preventDefault();
    app.key_up("arrowdown");
});
buttonLeft.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowleft");
});
buttonLeft.addEventListener("touchend", (event) => {
    event.preventDefault();
    app.key_up("arrowleft");
});
buttonRight.addEventListener("touchstart", (event) => {
    event.preventDefault();
    app.key_down("arrowright");
});
buttonRight.addEventListener("touchend", (event) => {
    event.preventDefault();
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
