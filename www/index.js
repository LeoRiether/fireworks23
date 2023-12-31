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
    fireworks.push_lerper(e.clientX, e.clientY, null, 1.0);
});

////////////////////////////////////////////////////////////////////

// Thanks, StackOverflow
// and past me (https://github.com/LeoRiether/Fireworks2019.5JS/blob/master/countdown.js)
let offset = 0;

const worldTimeAPI = {
    url: "https://worldtimeapi.org/api/timezone/Etc/UTC",
    timeParam: "utc_datetime",
};

const worldClockAPI = {
    url: "http://worldclockapi.com/api/json/utc/now",
    timeParam: "currentDateTime",
};

const anotherWorldTimeAPI = {
    url: "https://myworldtimeapi.herokuapp.com/Antarctica/Troll",
    timeParam: "utc_datetime",
};

const getOffsetWith = API => new Promise((res, rej) => {
    let xhr = new XMLHttpRequest();
    xhr.open("GET", API.url);
    xhr.responseType = 'json';

    xhr.onload = () => {
        let server = new Date(xhr.response[API.timeParam]);
        res(server - new Date());
    };

    xhr.onerror = rej;

    xhr.send();
});

const getOffset = () =>
    getOffsetWith(anotherWorldTimeAPI)
        .catch(() => {
            console.log("myworldtimeapi query failed! Trying worldclocktime instead");
            return getOffsetWith(worldTimeAPI);
        })
        .catch(() => {
            console.log("worldtimeapi query failed! Trying worldclockapi instead");
            return getOffsetWith(worldClockAPI);
        });

export function init_clock() {
    getOffset()
        .then(o => offset = o) // wtf don't set a global variable in async code like this
        .then(() => console.log('offset found: ', offset))
        .catch(() => console.log('getOffset() failed'));
}

init_clock();

export function date() {
    let d = new Date();
    d.setTime(d.getTime() + offset);
    return d;
}

////////////////////////////////////////////////////////////////////////////////

const targetDate = new Date(2024, 0, 1, 0, 0, 0, 0); // why the fuck are months 0-indexed?
let lastSTo = -1000;
function update_countdown() {
	let msTo = targetDate - date();
	let sTo = ~~(msTo / 1000);

    fireworks.update_countdown(sTo);
	
	if (lastSTo != 0) {
		setTimeout(update_countdown, 250);
	}
	lastSTo = sTo;
}

setTimeout(update_countdown, 250);

