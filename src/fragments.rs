use nalgebra_glm::Vec2;
use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Fragments {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
}

impl Fragments {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragments {
            position: Vec2::new(x, y),
            color,
            depth,
        }
    }
}