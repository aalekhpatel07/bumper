import init from "./web/bumper.js";
import { Car } from "./web/bumper.js";

let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

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

  let car = new Car(100, 100, 30, 50);
  registerKeyPresses(car);
  animate(car);
}

/**
 *
 * @param {Car} car
 */
function animate(car) {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  car.update();

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
