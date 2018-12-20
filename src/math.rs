use sdl2::rect::Point;
use std::f64::consts::PI;
use std::f64;

pub fn rotate(point_to_rotate: &Point, point_of_rotation: &Point, angle: &f64) -> Point{

    let x: f64 = angle.cos() * f64::from(point_to_rotate.x() - point_of_rotation.x()) - angle.sin() * f64::from(point_to_rotate.y() - point_of_rotation.y()) + f64::from(point_of_rotation.x());
    let y: f64 = angle.sin() * f64::from(point_to_rotate.x() - point_of_rotation.x()) - angle.cos() * f64::from(point_to_rotate.y() - point_of_rotation.y()) + f64::from(point_of_rotation.y());

    Point::new(x as i32,y as i32)
}

#[test]
fn rotate_test(){
    let angle = -PI/2.0;
    let point_origin = Point::new(3,4);
    let point_center = Point::new(3,3);
    let point_rotated_expected = Point::new(4,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    println!("point_origin: {:#?}",point_origin);
    println!("point_center: {:#?}",point_center);
    println!("point_rotated_expected: {:#?}",point_rotated_expected);
    println!("point_rotated: {:#?}",point_rotated);
    assert_eq!(point_rotated_expected, point_rotated);

    let angle = PI/2.0;
    let point_origin = Point::new(3,4);
    let point_center = Point::new(3,3);
    let point_rotated_expected = Point::new(2,3);
    let point_rotated = rotate(&point_origin, &point_center, &angle);
    println!("point_origin: {:#?}",point_origin);
    println!("point_center: {:#?}",point_center);
    println!("point_rotated_expected: {:#?}",point_rotated_expected);
    println!("point_rotated: {:#?}",point_rotated);
    assert_eq!(point_rotated_expected, point_rotated);
}
