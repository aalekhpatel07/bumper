//! This crate uses the canvas api and provides a way to draw and move a car
//! given some initial configuration like initial_speed, acceleration, friction, angle, etc.
//! 
//! The car can be moved with the arrow keys.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;


#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CarConfig {
    pub speed: f64,

    pub acceleration: f64,
    pub max_speed: f64,
    pub friction: f64,
    pub angle: f64,
    pub angle_delta: f64
}

impl Default for CarConfig {
    fn default() -> Self {
        CarConfig {
            speed: 0.0,
            acceleration: 0.2,
            max_speed: 3.0,
            friction: 0.05,
            angle: 0.0,
            angle_delta: 0.03,
        }
    }
}


impl std::fmt::Display for CarConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CarConfig {{ speed: {}, acceleration: {}, max_speed: {}, friction: {}, angle: {}, angle_delta: {} }}",
               self.speed, self.acceleration, self.max_speed, self.friction, self.angle, self.angle_delta)
    }
}

#[wasm_bindgen]
impl CarConfig {
    
    #[wasm_bindgen(constructor)]
    pub fn new(
        speed: f64,
        acceleration: f64,
        max_speed: f64,
        friction: f64,
        angle: f64,
        angle_delta: f64
    ) -> Self {
        CarConfig {
            speed,
            acceleration,
            max_speed,
            friction,
            angle,
            angle_delta,
        }
    }
}


#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Car {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub config: CarConfig,
    pub control: Control
}

#[wasm_bindgen]
impl Car {
    #[wasm_bindgen(js_name = "withConfig")]
    pub fn with_config(self, config: CarConfig) -> Self {
        Car {
            config,
            ..self
        }
    }
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Car {
        Car {
            x,
            y,
            width,
            height,
            config: CarConfig::default(),
            control: Control::default()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn get_forward(&self) -> bool {
        self.control.forward
    }

    #[wasm_bindgen(getter)]
    pub fn get_left(&self) -> bool {
        self.control.left
    }

    #[wasm_bindgen(getter)]
    pub fn get_right(&self) -> bool {
        self.control.right
    }

    #[wasm_bindgen(getter)]
    pub fn get_reverse(&self) -> bool {
        self.control.reverse
    }

    #[wasm_bindgen(setter)]
    pub fn set_forward(&mut self, forward: bool){
        self.control.forward = forward;
    }
    #[wasm_bindgen(setter)]
    pub fn set_left(&mut self, left: bool){
        self.control.left = left;
    }
    #[wasm_bindgen(setter)]
    pub fn set_right(&mut self, right: bool){
        self.control.right = right;
    }
    #[wasm_bindgen(setter)]
    pub fn set_reverse(&mut self, reverse: bool){
        self.control.reverse = reverse;
    }
}


impl std::fmt::Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Car {{ x: {}, y: {}, width: {}, height: {}, config: {} }}",
               self.x, self.y, self.width, self.height, self.config)
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Control {
    forward: bool,
    reverse: bool,
    left: bool,
    right: bool
}

#[wasm_bindgen]
impl Control {

    #[wasm_bindgen(getter)]
    pub fn get_forward(&self) -> bool {
        self.forward
    }

    #[wasm_bindgen(getter)]
    pub fn get_left(&self) -> bool {
        self.left
    }

    #[wasm_bindgen(getter)]
    pub fn get_right(&self) -> bool {
        self.right
    }

    #[wasm_bindgen(getter)]
    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
}

#[wasm_bindgen]
impl Car {
    pub fn update(&mut self) {

        if self.control.forward {
            self.config.speed += self.config.acceleration;
        } 
        if self.control.reverse {
            self.config.speed -= self.config.acceleration;
        }

        if self.config.speed > self.config.max_speed {
            self.config.speed = self.config.max_speed;
        }

        if self.config.speed < -self.config.max_speed/2. {
            self.config.speed = -self.config.max_speed/2.;
        }

        if self.config.speed > 0. {
            self.config.speed -= self.config.friction;
        }

        if self.config.speed < 0. {
            self.config.speed += self.config.friction;
        }

        if self.config.speed.abs() < self.config.friction {
            self.config.speed = 0.;
        }

        if self.config.speed != 0. {
            let flip = if self.config.speed > 0. { 1.} else { -1. };
            if self.control.left {
                self.config.angle += self.config.angle_delta * flip;
            }
            if self.control.right {
                self.config.angle -= self.config.angle_delta * flip;
            }
        }

        self.x -= self.config.angle.sin() * self.config.speed;
        self.y -= self.config.angle.cos() * self.config.speed;

    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        ctx.translate(self.x as f64, self.y as f64).unwrap();
        ctx.rotate(-self.config.angle).unwrap();

        const TIRE_WIDTH: f64 = 7.;
        const TIRE_HEIGHT: f64 = 14.;

        // Top left wheel.
        ctx.begin_path();
        ctx.rect(
            -self.width / 2. - TIRE_WIDTH / 2.,
            -self.height / 2. + TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();


        // Top right wheel.
        ctx.begin_path();
        ctx.rect(
            self.width / 2. - TIRE_WIDTH / 2.,
            -self.height / 2. + TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();


        // Bottom left wheel.
        ctx.begin_path();
        ctx.rect(
            -self.width / 2. - TIRE_WIDTH / 2.,
            self.height / 2. - 3. * TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();

        // Bottom right wheel.
        ctx.begin_path();
        ctx.rect(
            self.width / 2. - TIRE_WIDTH / 2.,
            self.height / 2. - 3. * TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();

        // Car body.
        ctx.begin_path();
        ctx.rect(
            -self.width / 2.,
            -self.height / 2.,
            self.width,
            self.height
        );
        ctx.set_fill_style(&"#659157".into());
        ctx.fill();

        ctx.restore();
    }
}
