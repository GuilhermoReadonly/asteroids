use sdl2::rect::Point;
use std::f64::consts::PI;
use std::f64;

pub fn rotate(point_to_rotate: &Point, point_of_rotation: &Point, angle: &f64) -> Point{

    let x: f64 = angle.cos() * f64::from(point_to_rotate.x() - point_of_rotation.x()) - angle.sin() * f64::from(point_to_rotate.y() - point_of_rotation.y()) + f64::from(point_of_rotation.x());
    let y: f64 = angle.sin() * f64::from(point_to_rotate.x() - point_of_rotation.x()) + angle.cos() * f64::from(point_to_rotate.y() - point_of_rotation.y()) + f64::from(point_of_rotation.y());

    Point::new(x.round() as i32,y.round() as i32)
}

#[test]
fn rotate_test(){
    let angle = -PI/2.0;
    let point_origin = Point::new(3,5);
    let point_center = Point::new(3,3);
    let point_rotated_expected = Point::new(5,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);

    let angle = PI/2.0;
    let point_origin = Point::new(3,5);
    let point_center = Point::new(3,3);
    let point_rotated_expected = Point::new(1,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);

    let angle = PI;
    let point_origin = Point::new(3,5);
    let point_center = Point::new(3,3);
    let point_rotated_expected = Point::new(3,1);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);

    let angle = 3.0 * PI / 2.0;
    let point_origin = Point::new(3,5);
    let point_center = Point::new(3,3);
    let point_rotated_expected = Point::new(5,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    assert_eq!(point_rotated_expected, point_rotated);
}
