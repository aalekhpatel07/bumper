use bumper_core::{Car, CarConfig, Control};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarView {
    // pub id: String,
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    pub config: CarConfig,
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub reverse: bool,
}

impl From<CarView> for Car {
    fn from(car_view: CarView) -> Self {
        Car {
            // id: car_view.id,
            x: car_view.x,
            y: car_view.y,
            config: car_view.config,
            height: car_view.height,
            width: car_view.width,
            control: Control {
                forward: car_view.forward,
                reverse: car_view.reverse,
                left: car_view.left,
                right: car_view.right,
            },
        }
    }
}

impl From<&CarView> for Car {
    fn from(car_view: &CarView) -> Self {
        Car {
            // id: car_view.id.clone(),
            x: car_view.x,
            y: car_view.y,
            config: car_view.config,
            height: car_view.height,
            width: car_view.width,
            control: Control {
                forward: car_view.forward,
                reverse: car_view.reverse,
                left: car_view.left,
                right: car_view.right,
            },
        }
    }
}
