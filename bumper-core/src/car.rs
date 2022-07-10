use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

///
///
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CarConfig {
    pub speed: f64,
    pub acceleration: f64,
    pub max_speed: f64,
    pub friction: f64,
    pub angle: f64,
    pub angle_delta: f64,
}

impl Default for CarConfig {
    fn default() -> Self {
        CarConfig {
            speed: 0.0,
            acceleration: 0.2,
            max_speed: 10.0,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    // pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub config: CarConfig,
    pub control: Control,
}


impl Car {
    pub fn with_config(self, config: CarConfig) -> Self {
        Car { config, ..self }
    }
    // pub fn with_id(self, id: String) -> Self {
    //     Car { id, ..self }
    // }
    pub fn new(
        x: f64, y: f64, width: f64, height: f64
    ) -> Car {
        Car {
            // id: Uuid::new_v4().to_string(),
            x,
            y,
            width,
            height,
            config: CarConfig::default(),
            control: Control::default(),
        }
    }

}


impl std::fmt::Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Car {{ x: {}, y: {}, width: {}, height: {}, config: {} }}",
            self.x, self.y, self.width, self.height, self.config
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarView {
    // pub id: String,
    pub x: f64,
    pub y: f64
}


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

        if self.config.speed < -self.config.max_speed / 2. {
            self.config.speed = -self.config.max_speed / 2.;
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
            let flip = if self.config.speed > 0. { 1. } else { -1. };
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

// }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Control {
    pub forward: bool,
    pub reverse: bool,
    pub left: bool,
    pub right: bool,
}
