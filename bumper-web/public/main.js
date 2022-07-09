import init from "./web/bumper.js";
import { Car, CarView } from "./web/bumper.js";

let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

let car;

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

  // let car = new Car(100, 100, 30, 50);
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
