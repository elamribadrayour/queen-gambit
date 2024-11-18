use nannou::color::Rgb;
use nannou::color::{BLACK, WHITE};

pub struct Square {
    pub x: f32,
    pub y: f32,
    pub color: Rgb<u8>,
}

impl Square {
    pub fn new(x: f32, y: f32, color: &str) -> Self {
        Self {
            x,
            y,
            color: if color == "black" { BLACK } else { WHITE },
        }
    }
}
