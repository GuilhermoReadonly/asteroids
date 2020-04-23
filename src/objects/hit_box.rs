#[derive(Debug, Clone)]
pub struct HitBox {
    pub width: f32,
    pub height: f32,
}

impl HitBox {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
