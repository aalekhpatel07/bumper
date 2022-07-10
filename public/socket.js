let worker;
let ws;

function attachListener() {
  let canvas = document.getElementById("canvas");
  if (canvas) {
    console.log("Attaching carMoved to canvas.");
    canvas.addEventListener("carMoved", (e) => {
      const newCar = e.detail;
      // console.log("from socket: newCar", newCar.toJSON());
      worker.postMessage(newCar.toJSON());
    });
  } else {
    console.log("No canvas found.");
  }
}

function onOpen(e) {
  console.log("Websocket open", e);
}

function onClose(e) {
  console.log("Websocket close", e);
}

/**
 *
 * @param {{ data: string }} message
 * @param {WebSocket} ws
 */
function onMessage(message) {
  let raw = message.data;
  let data = JSON.parse(raw);

  let car = data;
  // data.forEach();

  console.log("Websocket message: " + raw, data);
  console.log(car);

  setInterval(() => {
    ws.send(JSON.stringify({ hello: "world" }));
  }, 1000);
}

function onError(e) {
  console.error("Websocket error: " + e);
}

async function createConnection() {
  ws = new WebSocket("ws://localhost:8080/");
  console.log(ws);
  ws.onopen = onOpen;
  ws.onclose = onClose;
  ws.onmessage = onMessage;
  ws.onerror = onError;
}

function init() {
  createConnection();
  attachListener();
  worker = new Worker("workers/example.js");
}

init();
