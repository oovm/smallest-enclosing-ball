use shape_core::{Line, Triangle};
use smallest_enclosing_ball::EnclosingCircle;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn point2() {
    let disk = Line::new((0.0, 0.0), (1.0, 1.0)).enclosing().unwrap();
    println!("{:#?}", disk);
    let disk = Line::new((0.0, 0.0), (0.0, 0.0)).enclosing().unwrap();
    println!("{:#?}", disk);
}

#[test]
fn point3() {
    let disk = Triangle::new((0.0, 0.0), (1.0, 0.0), (0.0, 1.0)).enclosing().unwrap();
    println!("{:#?}", disk);
    let disk = Triangle::new((0.0, 0.0), (0.0, 0.0), (0.0, 0.0)).enclosing().unwrap();
    println!("{:#?}", disk);
}
