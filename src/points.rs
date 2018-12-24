use sdl2::rect::Point;

#[derive(Debug, PartialEq)]
pub struct PointExact {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq)]
pub struct PointWithOffset {
    pub point: Point,
    pub x_offset: i32,
    pub y_offset: i32,
}

impl PointWithOffset {
    pub fn new(point: Point, x_offset: i32, y_offset: i32) -> PointWithOffset {
        PointWithOffset{
            point,
            x_offset,
            y_offset
        }
    }
}
