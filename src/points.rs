use sdl2::rect::Point;

#[derive(Debug, PartialEq)]
pub struct PointExact {
    pub x: f64,
    pub y: f64,
}

impl PointExact {
    pub fn new(x: f64, y: f64) -> PointExact {
        PointExact{
            x,
            y,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PointWithOffset {
    pub point: Point,
    pub x_offset: i32,
    pub y_offset: i32,
}

impl PointWithOffset {
    pub fn new(x: i32, y: i32, x_offset: i32, y_offset: i32) -> PointWithOffset {
        let point = Point::new(x + x_offset, y + y_offset);
        PointWithOffset{
            point,
            x_offset,
            y_offset
        }
    }

    pub fn get_x_center(&self) -> i32{
        self.point.x() - self.x_offset
    }

    pub fn get_y_center(&self) -> i32{
        self.point.y() - self.y_offset
    }
}
