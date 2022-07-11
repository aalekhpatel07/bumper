// let worker;
let ws;
let canvas = document.getElementById("canvas");

function createEventDispatcher(elem) {
  return function (name, data) {
    let event = new CustomEvent(name, {
      detail: data,
    });
    elem.dispatchEvent(event);
  };
}

function attachListener() {
  if (canvas) {
    console.log("Attaching carMoved to canvas.");
    canvas.addEventListener("carMoved", (e) => {
      const newCar = e.detail;
      ws.send(JSON.stringify(newCar));
    });
  } else {
    console.log("No canvas found.");
  }
}

function onOpen(e) {
  console.log("Websocket open", e, "data", e.data);
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

  if (!Array.isArray(data)) {
    setTimeout(() => {
      const dispatch = createEventDispatcher(document.getElementById("canvas"));
      dispatch("cars", {
        initial: true,
        data,
      });
    }, 1000);
  } else {
    const dispatch = createEventDispatcher(document.getElementById("canvas"));
    dispatch("cars", {
      initial: false,
      data,
    });
  }

  // canvas.dispatchEvent("cars", {
  //   detail: {
  //     initial: false,
  //     data,
  //   },
  // });
  // if (data) {
  //   data.forEach(([car, id]) => {
  //     console.log("car", car);
  //     console.log("id", id);
  //   });
  // }
  // data.forEach();

  // console.log("Websocket message: " + raw, data);
  // console.log(car);
}

function onError(e) {
  console.error("Websocket error: " + e);
}

async function createConnection() {
  ws = new WebSocket("ws://localhost:8080/");
  ws.onopen = onOpen;
  ws.onclose = onClose;
  ws.onmessage = onMessage;
  ws.onerror = onError;
}

function init() {
  createConnection();
  attachListener();
  // worker = new Worker("workers/example.js");
}

init();
