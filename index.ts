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

function getTouchEventXY(event: TouchEvent): [Float64Array, Float64Array] {
    const x = [];
    const y = [];
    for (let i = 0; i < event.touches.length; i++) {
        x.push(event.touches[i].clientX);
        y.push(event.touches[i].clientY);
    }
    return [new Float64Array(x), new Float64Array(y)];
}

canvas.addEventListener('touchstart', (event) => {
    event.preventDefault();
    const [x, y] = getTouchEventXY(event);
    app.touch_start(new Date().getTime(), x, y);
});
canvas.addEventListener('touchmove', (event) => {
    event.preventDefault();
    const [x, y] = getTouchEventXY(event);
    app.touch_move(new Date().getTime(), x, y);
});
canvas.addEventListener('touchend', () => {
    app.touch_end(new Date().getTime());
});

let isClick = false;
canvas.addEventListener('mousedown', (event) => {
    isClick = true;
    app.touch_start(new Date().getTime(), new Float64Array([event.clientX]), new Float64Array([event.clientY]));
});
canvas.addEventListener('mousemove', (event) => {
    if (isClick) {
        app.touch_move(new Date().getTime(), new Float64Array([event.clientX]), new Float64Array([event.clientY]));
    }
});
canvas.addEventListener('mouseup', () => {
    isClick = false;
    app.touch_end(new Date().getTime());
});
canvas.addEventListener('mouseout', () => {
    isClick = false;
    app.touch_end(new Date().getTime());
});

const speedOfLightRange = document.getElementById("speed-of-light-exp") as HTMLInputElement;
const speedOfLightView = document.getElementById("speed-of-light") as HTMLSpanElement;
speedOfLightRange.onchange = () => {
    const e = speedOfLightRange.valueAsNumber;
    const c = Math.pow(2, e);
    speedOfLightView.innerText = c.toString();
    app.change_c(c);
};

const presetNodes = document.getElementsByName("preset") as NodeListOf<HTMLInputElement>;

const presetChange = () => {
    for (let i = 0; i < presetNodes.length; i++) {
        if (presetNodes.item(i).checked) {
            app.reset_charge(presetNodes.item(i).value);
            break;
        }
    }
};
const preset0 = document.getElementById("preset0") as HTMLSelectElement;
preset0.onchange = presetChange;
const preset1 = document.getElementById("preset1") as HTMLSelectElement;
preset1.onchange = presetChange;
const preset2 = document.getElementById("preset2") as HTMLSelectElement;
preset2.onchange = presetChange;
const preset3 = document.getElementById("preset3") as HTMLSelectElement;
preset3.onchange = presetChange;

const gridOptionNodes = document.getElementsByName("grid-option") as NodeListOf<HTMLInputElement>;

const gridOptionChange = () => {
    for (let i = 0; i < gridOptionNodes.length; i++) {
        if (gridOptionNodes.item(i).checked) {
            app.reset_grid(gridOptionNodes.item(i).value);
            gridOptionNodes.item(i)!.nextElementSibling!.classList.add("checked");
        } else {
            gridOptionNodes.item(i)!.nextElementSibling!.classList.remove("checked");
        }
    }
};
const go1 = document.getElementById("grid-option1") as HTMLSelectElement;
go1.onchange = gridOptionChange;
const go2 = document.getElementById("grid-option2") as HTMLSelectElement;
go2.onchange = gridOptionChange;

const arrowLog = document.getElementById("arrow-log") as HTMLInputElement;
arrowLog.onchange = () => {
    app.change_arrow_length_log(arrowLog.valueAsNumber);
};
const arrowFactor = document.getElementById("arrow-factor") as HTMLInputElement;
arrowFactor.onchange = () => {
    app.change_arrow_length_factor(Math.pow(2, arrowFactor.valueAsNumber));
};

app.change_arrow_length_log(arrowLog.valueAsNumber);
app.change_arrow_length_factor(Math.pow(2, arrowFactor.valueAsNumber));

function step(timestamp: DOMHighResTimeStamp): void {
    app.tick(timestamp);
    info.innerText = app.info();
    window.requestAnimationFrame(step);
}

window.requestAnimationFrame(step);
