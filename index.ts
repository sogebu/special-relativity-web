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
    if (event.key.toLowerCase().startsWith('arrow')) {
        event.preventDefault();
    }
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

const restartButton = document.getElementById("restart-button") as HTMLButtonElement;
const setRestarted = () => {
    restartButton.classList.add('clicked');
    setTimeout(() => {
        restartButton.classList.remove('clicked');
    }, 500);
};
restartButton.onclick = () => {
    app.restart_physics();
    setRestarted();
};


const speedOfLightRange = document.getElementById("speed-of-light-exp") as HTMLInputElement;
const speedOfLightView = document.getElementById("speed-of-light") as HTMLSpanElement;

function speedOfLight(): number {
    const e = speedOfLightRange.valueAsNumber;
    return Math.pow(2, e);
}

speedOfLightRange.onchange = () => {
    const c = speedOfLight();
    speedOfLightView.innerText = `${c * 2}`;
    if (app.change_c(c)) {
        setRestarted();
    }
};

const presetNodes = document.getElementsByName("preset") as NodeListOf<HTMLInputElement>;

function presetChange(): void {
    for (let i = 0; i < presetNodes.length; i++) {
        if (presetNodes.item(i).checked) {
            app.reset_charge(presetNodes.item(i).value);
            presetNodes.item(i)!.nextElementSibling!.classList.add("checked");
        } else {
            presetNodes.item(i)!.nextElementSibling!.classList.remove("checked");
        }
    }
}

for (let i = 0; i <= 5; i++) {
    const preset = document.getElementById(`preset${i}`) as HTMLSelectElement;
    preset.onchange = presetChange;
}

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

const poyntingNodes = document.getElementsByName("poynting") as NodeListOf<HTMLInputElement>;
const poyntingChange = () => {
    for (let i = 0; i < poyntingNodes.length; i++) {
        if (poyntingNodes.item(i).checked) {
            app.change_poynting_on(poyntingNodes.item(i).id === "poynting-on");
            poyntingNodes.item(i)!.nextElementSibling!.classList.add("checked");
        } else {
            poyntingNodes.item(i)!.nextElementSibling!.classList.remove("checked");
        }
    }
};
const poyntingOff = document.getElementById("poynting-off") as HTMLInputElement;
const poyntingOn = document.getElementById("poynting-on") as HTMLInputElement;
poyntingOff.onchange = poyntingChange;
poyntingOn.onchange = poyntingChange;

const arrowLog = document.getElementById("arrow-log") as HTMLInputElement;
const arrowLogPlus = document.getElementById("arrow-log-plus") as HTMLButtonElement;
const arrowLogMinus = document.getElementById("arrow-log-minus") as HTMLButtonElement;
arrowLogPlus.onclick = () => {
    const n = arrowLog.valueAsNumber + 1;
    app.change_arrow_length_log(n);
    arrowLog.value = `${n}`;
};
arrowLogMinus.onclick = () => {
    const n = arrowLog.valueAsNumber >= 1 ? arrowLog.valueAsNumber - 1 : 0;
    app.change_arrow_length_log(n);
    arrowLog.value = `${n}`;
};

const arrowFactor = document.getElementById("arrow-factor") as HTMLInputElement;
const arrowFactorPlus = document.getElementById("arrow-factor-plus") as HTMLButtonElement;
const arrowFactorMinus = document.getElementById("arrow-factor-minus") as HTMLButtonElement;
arrowFactorPlus.onclick = () => {
    const n = arrowFactor.valueAsNumber + 1;
    app.change_arrow_length_log(Math.pow(2, n));
    arrowFactor.value = `${n}`;
};
arrowFactorMinus.onclick = () => {
    const n = arrowFactor.valueAsNumber - 1;
    app.change_arrow_length_log(Math.pow(2, n));
    arrowFactor.value = `${n}`;
};

app.reset_charge(presetNodes.item(0).value);
app.change_c(speedOfLight());
app.change_arrow_length_log(arrowLog.valueAsNumber);
app.change_arrow_length_factor(Math.pow(2, arrowFactor.valueAsNumber));

function step(timestamp: DOMHighResTimeStamp): void {
    app.tick(timestamp);
    info.innerText = app.info();
    window.requestAnimationFrame(step);
}

window.requestAnimationFrame(step);
