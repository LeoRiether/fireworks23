import * as wasm from "fireworks23";

let fireworks = wasm.Fireworks.new();
const loop = () => {
    fireworks.tick();
    requestAnimationFrame(loop);
};
