// //! This crate uses the canvas api and provides a way to draw and move a car
// //! given some initial configuration like initial_speed, acceleration, friction, angle, etc.
// //!
// //! The car can be moved with the arrow keys.

// // use serde::{Deserialize, Serialize};
// use serde_derive::{Serialize, Deserialize};
// use wasm_bindgen::prelude::*;
// use web_sys::CanvasRenderingContext2d;
// use uuid::Uuid;
// use bumper_core::{Car as CarModel};

// #[wasm_bindgen(inspectable, js_name="CarConfig")]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CarConfig(car::CarConfig);

// #[wasm_bindgen(js_name="CarConfig")]
// impl CarConfig {
//     #[wasm_bindgen(constructor)]
//     pub fn new(
//         speed: f64,
//         acceleration: f64,
//         max_speed: f64,
//         friction: f64,
//         angle: f64,
//         angle_delta: f64,
//     ) -> Self {
//         CarConfig(car::CarConfig {
//             speed,
//             acceleration,
//             max_speed,
//             friction,
//             angle,
//             angle_delta,
//         })
//     }
// }

// #[wasm_bindgen(inspectable)]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Car(pub(crate) car::Car);

// #[wasm_bindgen(js_name="CarView")]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CarView(car::CarView);

// #[wasm_bindgen(js_name="CarView")]
// impl CarView {
//     #[wasm_bindgen(constructor)]
//     pub fn new(id: &str, x: f64, y: f64) -> Self {
//         CarView(
//             car::CarView { id: id.into(), x, y }
//         )
//     }
//     #[wasm_bindgen(getter)]
//     pub fn id(&self) -> String {
//         self.0.id.clone()
//     }
// }

// #[wasm_bindgen(js_name="Control")]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
// pub struct Control(car::Control);

// #[wasm_bindgen(js_name="Control")]
// impl Control {
//     #[wasm_bindgen(getter)]
//     pub fn forward(&self) -> bool {
//         self.0.forward
//     }

//     #[wasm_bindgen(getter)]
//     pub fn left(&self) -> bool {
//         self.0.left
//     }

//     #[wasm_bindgen(getter)]
//     pub fn right(&self) -> bool {
//         self.0.right
//     }

//     #[wasm_bindgen(getter)]
//     pub fn reverse(&self) -> bool {
//         self.0.reverse
//     }

// }

// #[wasm_bindgen(js_name="Car")]
// impl Car {

//     #[wasm_bindgen(constructor)]
//     pub fn new(
//         x: f64, y: f64, width: f64, height: f64
//     ) -> Car {
//         Car(car::Car {
//             id: Uuid::new_v4().to_string(),
//             x,
//             y,
//             width,
//             height,
//             config: car::CarConfig::default(),
//             control: car::Control::default(),
//         })
//     }

//     pub fn as_view(&self) -> CarView {
//         let car_view: car::CarView = (&self.0).into();
//         CarView(car_view)
//     }

//     pub fn update(&mut self) {
//         self.0.update()
//     }

//     pub fn draw(&mut self, ctx: &CanvasRenderingContext2d) {
//         self.0.draw(ctx)
//     }

//     #[wasm_bindgen(getter)]
//     pub fn id(&self) -> String {
//         self.0.id.clone()
//     }

//     #[wasm_bindgen(getter)]
//     pub fn forward(&self) -> bool {
//         self.0.control.forward
//     }

//     #[wasm_bindgen(getter)]
//     pub fn left(&self) -> bool {
//         self.0.control.left
//     }

//     #[wasm_bindgen(getter)]
//     pub fn right(&self) -> bool {
//         self.0.control.right
//     }

//     #[wasm_bindgen(getter)]
//     pub fn reverse(&self) -> bool {
//         self.0.control.reverse
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_forward(&mut self, forward: bool) {
//         self.0.control.forward = forward;
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_left(&mut self, left: bool) {
//         self.0.control.left = left;
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_right(&mut self, right: bool) {
//         self.0.control.right = right;
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_reverse(&mut self, reverse: bool) {
//         self.0.control.reverse = reverse;
//     }
// }



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
