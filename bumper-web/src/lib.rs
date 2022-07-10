// //! This crate uses the canvas api and provides a way to draw and move a car
// //! given some initial configuration like initial_speed, acceleration, friction, angle, etc.
// //!
// //! The car can be moved with the arrow keys.

// // use serde::{Deserialize, Serialize};
use serde_derive::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use uuid::Uuid;
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
}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car(bumper_core::Car);

#[wasm_bindgen(js_name="Car")]
impl Car {

    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f64, y: f64, width: f64, height: f64
    ) -> Car {
        Car(bumper_core::Car {
            id: Uuid::new_v4().to_string(),
            x,
            y,
            width,
            height,
            config: bumper_core::CarConfig::default(),
            control: bumper_core::Control::default(),
        })
    }

    pub fn update(&mut self) {
        self.0.update()
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        ctx.translate(self.0.x as f64, self.0.y as f64).unwrap();
        ctx.rotate(-self.0.config.angle).unwrap();

        const TIRE_WIDTH: f64 = 7.;
        const TIRE_HEIGHT: f64 = 14.;

        // Top left wheel.
        ctx.begin_path();
        ctx.rect(
            -self.0.width / 2. - TIRE_WIDTH / 2.,
            -self.0.height / 2. + TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT,
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();

        // Top right wheel.
        ctx.begin_path();
        ctx.rect(
            self.0.width / 2. - TIRE_WIDTH / 2.,
            -self.0.height / 2. + TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT,
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();

        // Bottom left wheel.
        ctx.begin_path();
        ctx.rect(
            -self.0.width / 2. - TIRE_WIDTH / 2.,
            self.0.height / 2. - 3. * TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT,
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();

        // Bottom right wheel.
        ctx.begin_path();
        ctx.rect(
            self.0.width / 2. - TIRE_WIDTH / 2.,
            self.0.height / 2. - 3. * TIRE_HEIGHT / 2.,
            TIRE_WIDTH,
            TIRE_HEIGHT,
        );
        ctx.set_fill_style(&"#000000".into());
        ctx.fill();

        // Car body.
        ctx.begin_path();
        ctx.rect(-self.0.width / 2., -self.0.height / 2., self.0.width, self.0.height);
        ctx.set_fill_style(&"#659157".into());
        ctx.fill();

        ctx.restore();
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.0.id.clone()
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