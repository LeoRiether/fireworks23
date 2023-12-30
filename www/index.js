import * as wasm from "fireworks23";

console.log("[JS] initializing fireworks23");
wasm.ping();

let fireworks = wasm.Fireworks.new();
const loop = () => {
    fireworks.tick();
    requestAnimationFrame(loop);
};
requestAnimationFrame(loop);
