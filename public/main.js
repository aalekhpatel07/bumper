import init from "./web/bumper_web.js";
import { Car } from "./web/bumper_web.js";

let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

let car;
let currentPos;
let prevPos;

function dispatchCarMove(c) {
  const event = new CustomEvent("carMoved", { detail: c });
  // console.log("car moved! prevPos:", prevPos, "currentPos:", currentPos);
  canvas.dispatchEvent(event);
}

/**
 *
 * @param {Car} car
 */
function registerKeyPresses(car) {
  window.onkeydown = (event) => {
    switch (event.key) {
      case "ArrowLeft":
        car.left = true;
        break;
      case "ArrowRight":
        car.right = true;
        break;
      case "ArrowUp":
        car.forward = true;
        break;
      case "ArrowDown":
        car.reverse = true;
        break;
    }
  };

  window.onkeyup = (event) => {
    switch (event.key) {
      case "ArrowLeft":
        car.left = false;
        break;
      case "ArrowRight":
        car.right = false;
        break;
      case "ArrowUp":
        car.forward = false;
        break;
      case "ArrowDown":
        car.reverse = false;
        break;
    }
  };
}

async function onInit() {
  await setup();
  // canvas.dispatchEvent();

  car = new Car(100, 100, 30, 50);
  currentPos = { x: car.x, y: car.y };
  prevPos = { ...currentPos };
  // console.log(car.id);
  // let view = car.as_view();
  // console.log(view.toString());
  setTimeout(() => {
    registerKeyPresses(car);
    animate(car);
  }, 5000);
}

/**
 *
 * @param {Car} car
 */
function animate(car) {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  prevPos = { ...currentPos };
  car.update();
  currentPos = { x: car.x, y: car.y };

  // console.log("")
  if (!!(currentPos.x === prevPos.x && currentPos.y === prevPos.y)) {
    // console.log("currentPos:", currentPos, "prevPos:", prevPos);
    dispatchCarMove(car);
  }
  car.draw(ctx);

  requestAnimationFrame(() => animate(car));
}

async function setup() {
  let canvas = document.getElementById("canvas");
  canvas.style.backgroundColor = "maroon";
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

window.onload = async () => {
  await init();
  await onInit();
};
