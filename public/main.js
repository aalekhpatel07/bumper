import init from "./web/bumper_web.js";
import { Car, CarPosition } from "./web/bumper_web.js";

let backgroundCanvas = document.getElementById("background");
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

let cars = new Map();

let car;
let currentPos;
let prevPos;

function dispatchCarMove(c) {
  const event = new CustomEvent("carMoved", { detail: c });
  // console.log("Dispatch fired:", c);
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
  // car = new Car(100, 100, 30, 50);
  // console.log(car.id);
  // let view = car.as_view();
  // console.log(view.toString());
  setTimeout(() => {
    animate();
  }, 5000);
}

/**
 *
 */
function animate() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  if (car) {
    draw(car, ctx);
    car.update();
    currentPos.x = car.x;
    currentPos.y = car.y;
  }

  if (cars) {
    drawAllCars(cars, ctx);
  }
  requestAnimationFrame(animate);
}

/**
 *
 * @param {HTMLCanvasElement} canvas
 */
function resizeCanvas(canvas) {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

/**
 *
 * @param {HTMLVideoElement} video
 * @param {CanvasRenderingContext2D} ctx
 */
function drawVideo(video, ctx) {
  if (video.paused || video.ended) return;
  ctx.drawImage(video, 0, 0, video.videoWidth / 2, video.videoHeight / 2);

  requestAnimationFrame(() => drawVideo(video, ctx));
}

async function setup() {
  let canvas = document.getElementById("canvas");
  let video = document.getElementById("video");
  let videoCanvas = document.getElementById("videoCanvas");

  // backgroundCanvas.style.backgroundColor = "maroon";
  resizeCanvas(backgroundCanvas);

  canvas.style.backgroundColor = "transparent";
  resizeCanvas(canvas);

  resizeCanvas(videoCanvas);
  resizeCanvas(video);

  video.addEventListener("play", () => {
    requestAnimationFrame(() => drawVideo(video, videoCanvas));
  });

  canvas.addEventListener("cars", (e) => {
    const { initial, data } = e.detail;
    // debugger;
    console.log("initial", initial);
    if (initial) {
      car = new Car(data.x, data.y, data.width, data.height);
      registerKeyPresses(car);
      prevPos = { x: car.x, y: car.y };
      currentPos = new Proxy(
        {
          x: car.x,
          y: car.y,
        },
        {
          set: function (target, key, value) {
            if (prevPos.x !== target.x || prevPos.y !== target.y) {
              dispatchCarMove(car);
              cars.forEach((v, ...rest) => {
                console.log("v: ", v);

                let position = new CarPosition(
                  v.x,
                  v.y,
                  v.width,
                  v.height,
                  v.config.angle
                );
                if (car.collidesPosition(position)) {
                  console.log("Car", car, "collides", v);
                }
              });
              prevPos = {
                x: target.x,
                y: target.y,
              };
            }
            target[key] = value;
            return true;
          },
          get: function (target, key) {
            return target[key];
          },
        }
      );

      // console.log("Setting car:", car);
    } else {
      // console.log("Not initial:", e.detail);

      cars = new Map();
      data.forEach((player) => {
        // console.log("peerId", peerId, "peerCar", peerCar);
        cars.set(player.id, player.car);
        // cars.insert(peerId, peerCar);
        // registerKeyPresses(peerCar);
      });
      // console.log("cars", cars);
    }
  });
}

window.onload = async () => {
  await init();
  await onInit();
};

/**
 *
 * @param {Map<string, Car>} allCars
 */
function drawAllCars(allCars, ctx) {
  allCars.forEach((value, ...rest) => {
    draw(value, ctx);
  });
  // allCars
  // .values()

  // for (let stuff in allCars.entries()) {
  //   console.log("drawAllCars stuff:", stuff);
  //   draw(stuff[1], ctx);
  // }
  // allCars.entries().forEach(([peerCarId, peerCar]) => {
  //   draw(peerCar, ctx);
  // });
}

/**
 *
 * @param {Car} car
 * @param {CanvasRenderingContext2D} ctx
 */
function draw(car, ctx) {
  let { x, y, width, height } = car;

  ctx.save();
  ctx.translate(x, y);
  ctx.rotate(-car.config.angle);

  const TIRE_WIDTH = 7;
  const TIRE_HEIGHT = 14;

  // Top left wheel.
  ctx.beginPath();
  ctx.fillStyle = "#000000";
  ctx.fillRect(
    -width / 2 - TIRE_WIDTH / 2,
    -height / 2 + TIRE_HEIGHT / 2,
    TIRE_WIDTH,
    TIRE_HEIGHT
  );

  // Top right wheel.
  ctx.beginPath();
  ctx.fillStyle = "#000000";
  ctx.fillRect(
    width / 2 - TIRE_WIDTH / 2,
    -height / 2 + TIRE_HEIGHT / 2,
    TIRE_WIDTH,
    TIRE_HEIGHT
  );

  // Bottom right wheel.
  ctx.beginPath();
  ctx.fillStyle = "#000000";
  ctx.fillRect(
    width / 2 - TIRE_WIDTH / 2,
    height / 2 - (3 * TIRE_HEIGHT) / 2,
    TIRE_WIDTH,
    TIRE_HEIGHT
  );

  // Bottom left wheel.
  ctx.beginPath();
  ctx.fillStyle = "#000000";
  ctx.fillRect(
    -width / 2 - TIRE_WIDTH / 2,
    height / 2 - (3 * TIRE_HEIGHT) / 2,
    TIRE_WIDTH,
    TIRE_HEIGHT
  );

  // Car body
  ctx.beginPath();
  ctx.fillStyle = "#659157";
  ctx.fillRect(-width / 2, -height / 2, width, height);

  ctx.restore();
}