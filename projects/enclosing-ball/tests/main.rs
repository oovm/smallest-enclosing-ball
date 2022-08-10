use shape_core::Point3D;

#[test]
fn ready() {
    println!("it works!")
}


fn main() {
    let points = vec![
        Point3D { x: 1.0, y: 2.0, z: 3.0 },
        Point3D { x: -1.0, y: 5.0, z: -2.0 },
        Point3D { x: 0.0, y: 0.0, z: 0.0 },
        Point3D { x: 0.0, y: 0.0, z: 1.0 },
        Point3D { x: 0.0, y: 1.0, z: 0.0 },
        Point3D { x: 1.0, y: 0.0, z: 0.0 },
    ];
}