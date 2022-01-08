import {App} from "special-relativity-web";

const canvas = document.getElementById("canvas") as HTMLCanvasElement | null;
if (!canvas) {
  throw new Error("No 'canvas'");
}
canvas.width = 800;
canvas.height = 600;
const context = canvas.getContext("webgl2");
if (!context) {
  throw new Error("webgl2 not supported");
}
context.viewport(0, 0, canvas.width, canvas.height);

const app = new App(context);

function step(timestamp: DOMHighResTimeStamp): void {
  app.tick(timestamp);
  window.requestAnimationFrame(step);
}

window.requestAnimationFrame(step);
