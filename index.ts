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
    canvas.width = 1920;
    canvas.height = 1080;
}

const context = canvas.getContext("webgl2");
if (!context) {
    throw new Error("webgl2 not supported");
}
context.viewport(0, 0, canvas.width, canvas.height);

const info = document.getElementById("info")!;

const app = new App(context);

type Model = {
    speedOfLight: number;
    electricOn: boolean;
    magneticOn: boolean;
    poyntingOn: boolean;
    arrowLog: number;
    arrowFactor: number;
};
const defaultModel: Model = {
    speedOfLight: 2,
    electricOn: true,
    magneticOn: true,
    poyntingOn: false,
    arrowLog: 1,
    arrowFactor: 2,
};
const initModel = {
    eom_with_static: defaultModel,
    eom: defaultModel,
    static: {
        ...defaultModel,
        speedOfLight: 0,
    },
    line_o: defaultModel,
    circle: {
        ...defaultModel,
        electricOn: false,
    },
} as const;


const takeScreenShot = (): void => {
    canvas.toBlob((blob) => {
        if (!blob) {
            console.error('Fail to get canvas blob data');
            return;
        }
        const item = new ClipboardItem({'image/png': blob});
        navigator.clipboard.write([item]).catch((error) => {
            console.error('Failed to copy to clipboard: ', error);
        });
    });
};

document.addEventListener('keydown', (event) => {
    const key = event.key.toLowerCase();
    if (key.startsWith('arrow')) {
        event.preventDefault();
    }
    app.key_down(key);
});
document.addEventListener('keyup', (event) => {
    app.key_up(event.key.toLowerCase());
});
window.addEventListener('blur', () => {
    app.window_blue();
});

const getTouchEventXY = (event: TouchEvent): [Float64Array, Float64Array] => {
    const x = [];
    const y = [];
    for (let i = 0; i < event.touches.length; i++) {
        x.push(event.touches[i].clientX);
        y.push(event.touches[i].clientY);
    }
    return [new Float64Array(x), new Float64Array(y)];
};

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
    if (event.ctrlKey) {
        takeScreenShot();
        isClick = false;
    }
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

const speedOfLight = (): number => {
    const e = speedOfLightRange.valueAsNumber;
    return Math.pow(2, e);
}
const changeSpeedOfLight = (): void => {
    const c = speedOfLight();
    speedOfLightView.innerText = `${c}`;
    if (app.change_c(c)) {
        setRestarted();
    }
}
speedOfLightRange.onchange = changeSpeedOfLight;

const lorentz = document.getElementById('lorentz') as HTMLInputElement;
lorentz.onchange = (): void => {
    app.change_correct_lorentz(!lorentz.checked);
}

const presetNodes = document.getElementsByName("preset") as NodeListOf<HTMLInputElement>;

const presetChange = (): void => {
    for (let i = 0; i < presetNodes.length; i++) {
        if (presetNodes.item(i).checked) {
            resetModel(presetNodes.item(i).value as keyof typeof initModel);
            presetNodes.item(i)!.nextElementSibling!.classList.add("checked");
        } else {
            presetNodes.item(i)!.nextElementSibling!.classList.remove("checked");
        }
    }
}

for (let i = 0; i < presetNodes.length; i++) {
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

const electricToggle = document.getElementById('electric-toggle') as HTMLInputElement;
const magneticToggle = document.getElementById('magnetic-toggle') as HTMLInputElement;
const poyntingToggle = document.getElementById('poynting-toggle') as HTMLInputElement;
const setElectricToggle = (on: boolean): void => {
    electricToggle.checked = on;
    app.change_electric_on(on);
};
const setMagneticToggle = (on: boolean): void => {
    magneticToggle.checked = on;
    app.change_magnetic_on(on);
};
const setPoyntingToggle = (on: boolean): void => {
    poyntingToggle.checked = on;
    app.change_poynting_on(on);
};
electricToggle.onchange = () => {
    app.change_electric_on(electricToggle.checked);
};
magneticToggle.onchange = () => {
    app.change_magnetic_on(magneticToggle.checked);
};
poyntingToggle.onchange = () => {
    app.change_poynting_on(poyntingToggle.checked);
};

const arrowLog = document.getElementById("arrow-log") as HTMLInputElement;
const arrowLogPlus = document.getElementById("arrow-log-plus") as HTMLButtonElement;
const arrowLogMinus = document.getElementById("arrow-log-minus") as HTMLButtonElement;
const setArrowLog = (n: number): void => {
    app.change_arrow_length_log(n);
    arrowLog.value = `${n}`;
};
arrowLogPlus.onclick = () => {
    const n = arrowLog.valueAsNumber + 1;
    setArrowLog(n);
};
arrowLogMinus.onclick = () => {
    const n = arrowLog.valueAsNumber >= 1 ? arrowLog.valueAsNumber - 1 : 0;
    setArrowLog(n);
};

const arrowFactor = document.getElementById("arrow-factor") as HTMLInputElement;
const arrowFactorPlus = document.getElementById("arrow-factor-plus") as HTMLButtonElement;
const arrowFactorMinus = document.getElementById("arrow-factor-minus") as HTMLButtonElement;
const setArrowFactor = (n: number): void => {
    app.change_arrow_length_factor(Math.pow(10, n));
    arrowFactor.value = `${n}`;
};
arrowFactorPlus.onclick = () => {
    const n = arrowFactor.valueAsNumber + 1;
    setArrowFactor(n);
};
arrowFactorMinus.onclick = () => {
    const n = arrowFactor.valueAsNumber - 1;
    setArrowFactor(n);
};

const resetModel = (preset: keyof typeof initModel) => {
    const m = initModel[preset];
    speedOfLightRange.valueAsNumber = m.speedOfLight;
    changeSpeedOfLight();
    setArrowLog(m.arrowLog);
    setArrowFactor(m.arrowFactor);
    setElectricToggle(m.electricOn);
    setMagneticToggle(m.magneticOn);
    setPoyntingToggle(m.poyntingOn);
    app.reset_charge(preset);
}

resetModel(presetNodes.item(0).value as keyof typeof initModel);

const step = (timestamp: DOMHighResTimeStamp): void => {
    app.tick(timestamp);
    info.innerText = app.info();
    window.requestAnimationFrame(step);
}

window.requestAnimationFrame(step);
