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
function onMessage(message, ws = undefined) {
  let raw = message.data;
  let data = JSON.parse(raw);

  let car = data;
  data.forEach();

  console.log("Websocket message: " + raw, data);

  setInterval(() => {
    ws.send(JSON.stringify({ hello: "world" }));
  }, 1000);
}

function onError(e) {
  console.error("Websocket error: " + e);
}

async function createConnection() {
  let ws = new WebSocket("ws://localhost:8001/");

  ws.onopen = onOpen;
  ws.onclose = onClose;
  ws.onmessage = (e) => onMessage(e, ws);
  ws.onerror = onError;
}

async function init() {
  await createConnection();
}

window.onload = init;
