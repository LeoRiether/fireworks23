import * as wasm from "fireworks23";

console.log("[JS] initializing fireworks23");

let fireworks = wasm.Fireworks.new();
// app.ticker.add(_ => fireworks.tick());

const loop = () => {
    fireworks.tick();
    requestAnimationFrame(loop);
};
requestAnimationFrame(loop);

window.addEventListener("click", e => {
    fireworks.push_lerper(e.clientX, e.clientY, 1.0);
});
