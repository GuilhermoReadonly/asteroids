use sdl2::rect::Point;
use std::f64;
use crate::points::PointExact;

pub fn rotate(point_to_rotate: &Point, point_of_rotation: &PointExact, angle: &f64) -> Point{

    let x: f64 = (-angle).cos() * (point_to_rotate.x() as f64 - point_of_rotation.x) - (-angle).sin() * (point_to_rotate.y() as f64 - point_of_rotation.y) + point_of_rotation.x;
    let y: f64 = (-angle).sin() * (point_to_rotate.x() as f64 - point_of_rotation.x) + (-angle).cos() * (point_to_rotate.y() as f64 - point_of_rotation.y) + point_of_rotation.y;

    Point::new(x.round() as i32,y.round() as i32)
}

pub fn translate(point_to_translate: &PointExact, step: &f64, angle: &f64) -> PointExact{

    let x: f64 = -angle.sin() * step + point_to_translate.x as f64;
    let y: f64 = -angle.cos() * step + point_to_translate.y as f64;

    PointExact{x: x, y: y,}
}

#[test]
fn translate_test(){
    use std::f64::consts::PI;
    let angle = 0.0;
    let point_origin = PointExact{x: 3.0, y: 5.0,};
    let step = 1.0;
    let point_translated_expected = PointExact{x: 3.0, y: 4.0};
    let point_translated = translate(&point_origin, &step, &angle);
    assert_eq!(point_translated_expected, point_translated);

    let angle = 0.0;
    let point_origin = PointExact{x: 3.0, y: 5.0,};
    let step = -1.0;
    let point_translated_expected = PointExact{x: 3.0, y: 6.0};
    let point_translated = translate(&point_origin, &step, &angle);
    assert_eq!(point_translated_expected, point_translated);

    let angle = PI;
    let point_origin = PointExact{x: 3.0, y: 5.0,};
    let step = 1.0;
    let point_translated_expected = PointExact{x: 3.0, y: 6.0};
    let point_translated = translate(&point_origin, &step, &angle);
    assert_eq!(point_translated_expected, point_translated);

    let angle = PI/2.0;
    let point_origin = PointExact{x: 3.0, y: 5.0,};
    let step = 1.0;
    let point_translated_expected = PointExact{x: 2.0, y: 5.0};
    let point_translated = translate(&point_origin, &step, &angle);
    assert_eq!(point_translated_expected, point_translated);

    let angle = -PI/2.0;
    let point_origin = PointExact{x: 3.0, y: 5.0,};
    let step = 1.0;
    let point_translated_expected = PointExact{x: 4.0, y: 5.0};
    let point_translated = translate(&point_origin, &step, &angle);
    assert_eq!(point_translated_expected, point_translated);

    let angle = PI/4.0;
    let point_origin = PointExact{x: 3.0, y: 5.0,};
    let step = 2.0_f64.sqrt();
    let point_translated_expected = PointExact{x: 2.0, y: 4.0};
    let point_translated = translate(&point_origin, &step, &angle);
    assert_eq!(point_translated_expected, point_translated);
}

#[test]
fn rotate_test(){
    use std::f64::consts::PI;
    let angle = -PI/2.0;
    let point_origin = Point::new(3,5);
    let point_center = PointExact{x: 3.0, y: 3.0,};
    let point_rotated_expected = Point::new(1,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);

    let angle = PI/2.0;
    let point_origin = Point::new(3,5);
    let point_center = PointExact{x: 3.0, y: 3.0,};
    let point_rotated_expected = Point::new(5,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);

    let angle = PI;
    let point_origin = Point::new(3,5);
    let point_center = PointExact{x: 3.0, y: 3.0,};
    let point_rotated_expected = Point::new(3,1);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);

    let angle = 3.0 * PI / 2.0;
    let point_origin = Point::new(3,5);
    let point_center = PointExact{x: 3.0, y: 3.0,};
    let point_rotated_expected = Point::new(1,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);
}
