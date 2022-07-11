// //! This crate uses the canvas api and provides a way to draw and move a car
// //! given some initial configuration like initial_speed, acceleration, friction, angle, etc.
// //!
// //! The car can be moved with the arrow keys.

// // use serde::{Deserialize, Serialize};
use serde_derive::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use bumper_core;


#[wasm_bindgen(inspectable, js_name="CarConfig")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarConfig(bumper_core::CarConfig);


#[wasm_bindgen(inspectable, js_name="CarConfig")]
impl CarConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        speed: f64,
        acceleration: f64,
        max_speed: f64,
        friction: f64,
        angle: f64,
        angle_delta: f64,
    ) -> Self {
        CarConfig(bumper_core::CarConfig {
            speed,
            acceleration,
            max_speed,
            friction,
            angle,
            angle_delta,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn speed(&self) -> f64 {
        self.0.speed
    }

    #[wasm_bindgen(getter)]
    pub fn acceleration(&self) -> f64 {
        self.0.acceleration
    }
    #[wasm_bindgen(getter)]
    pub fn max_speed(&self) -> f64 {
        self.0.max_speed
    }
    #[wasm_bindgen(getter)]
    pub fn friction(&self) -> f64 {
        self.0.friction
    }
    #[wasm_bindgen(getter)]
    pub fn angle(&self) -> f64 {
        let val = self.0.angle.rem_euclid(std::f64::consts::TAU) - std::f64::consts::TAU;
        if (val - std::f64::consts::TAU).abs() < 0.1 {
            0.0
        } else {
            val
        }
    }
    #[wasm_bindgen(getter)]
    pub fn angle_delta(&self) -> f64 {
        self.0.angle_delta
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarView(bumper_core::CarView);

#[wasm_bindgen(js_name="CarView")]
impl CarView {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        config: CarConfig,
        left: bool,
        right: bool,
        forward: bool,
        reverse: bool,
    ) -> Self {
        CarView(bumper_core::CarView {
            x,
            y,
            width,
            height,
            config: config.0,
            left,
            right,
            forward,
            reverse,
        })
    }
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.0.x
    }
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.0.y
    }
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 {
        self.0.width
    }
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 {
        self.0.height
    }
    #[wasm_bindgen(getter)]
    pub fn config(&self) -> CarConfig {
        CarConfig(self.0.config)
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car(bumper_core::Car);


#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarPosition(bumper_core::CarPosition);
#[wasm_bindgen(js_name="CarPosition")]
impl CarPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64, angle: f64) -> Self {
        CarPosition(bumper_core::CarPosition {
            x,
            y,
            width,
            height,
            angle,
        })
    }
}


#[wasm_bindgen(js_name="Car")]
impl Car {

    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f64, y: f64, width: f64, height: f64
    ) -> Car {
        Car(bumper_core::Car {
            // id: Uuid::new_v4().to_string(),
            x,
            y,
            width,
            height,
            config: bumper_core::CarConfig::default(),
            control: bumper_core::Control::default(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn config(&self) -> CarConfig {
        CarConfig(self.0.config)
    }

    pub fn update(&mut self) {
        self.0.update()
    }

    pub fn collides(&self, car: &Car) -> bool {
        self.0.collides(&car.0)
    }

    #[wasm_bindgen(js_name="collidesPosition")]
    pub fn collides_position(&self, car_position: &CarPosition) -> bool {
        self.0.collides_position(&car_position.0)
    }

    #[wasm_bindgen(js_name="collidesCarView")]
    pub fn collides_car_view(&self, car_view: &CarView) -> bool {
        self.0.collides_car_view(&car_view.0)
    }

    // #[wasm_bindgen(getter)]
    // pub fn id(&self) -> String {
    //     self.0.id.clone()
    // }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 {
        self.0.height
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 {
        self.0.width
    }

    #[wasm_bindgen(getter)]
    pub fn forward(&self) -> bool {
        self.0.control.forward
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.0.y
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.0.x
    }

    #[wasm_bindgen(getter)]
    pub fn left(&self) -> bool {
        self.0.control.left
    }

    #[wasm_bindgen(getter)]
    pub fn right(&self) -> bool {
        self.0.control.right
    }

    #[wasm_bindgen(getter)]
    pub fn reverse(&self) -> bool {
        self.0.control.reverse
    }

    #[wasm_bindgen(setter)]
    pub fn set_forward(&mut self, forward: bool) {
        self.0.control.forward = forward;
    }

    #[wasm_bindgen(setter)]
    pub fn set_left(&mut self, left: bool) {
        self.0.control.left = left;
    }

    #[wasm_bindgen(setter)]
    pub fn set_right(&mut self, right: bool) {
        self.0.control.right = right;
    }

    #[wasm_bindgen(setter)]
    pub fn set_reverse(&mut self, reverse: bool) {
        self.0.control.reverse = reverse;
    }
}