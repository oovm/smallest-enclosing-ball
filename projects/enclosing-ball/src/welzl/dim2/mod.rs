use std::f64;

// 2D点的结构体
#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

// 2D圆的结构体
#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

// 计算两点之间的距离
fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    f64::sqrt(dx * dx + dy * dy)
}

// 判断点是否在圆内
fn point_in_circle(point: &Point, circle: &Circle) -> bool {
    distance(point, &circle.center) <= circle.radius
}

// Welzl算法的主要实现函数
fn welzl(points: &[Point], boundary: &[Point]) -> Circle {
    if boundary.len() == 3 {
        // 当边界上有三个点时，直接构建一个圆
        let circle = circumcircle(&boundary[0], &boundary[1], &boundary[2]);
        if points.iter().all(|p| point_in_circle(p, &circle)) {
            return circle;
        }
    }

    if points.is_empty() || boundary.len() == 3 {
        // 当没有点或边界上有三个点时，返回一个包含边界点的圆
        let center = calculate_center(&boundary);
        let radius = boundary.iter().map(|p| distance(p, &center)).fold(0.0, f64::max);
        return Circle { center, radius };
    }

    let point = &points[0];
    let remaining_points = &points[1..];

    // 递归调用Welzl算法，不包含当前点
    let circle = welzl(remaining_points, boundary);
    if point_in_circle(point, &circle) {
        return circle;
    }

    // 递归调用Welzl算法，包含当前点
    let mut new_boundary = boundary.to_vec();
    new_boundary.push(point.clone());
    welzl(remaining_points, &new_boundary)
}

// 计算三个点确定的圆
fn circumcircle(p1: &Point, p2: &Point, p3: &Point) -> Circle {
    let d = 2.0 * (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y));
    let ux = ((p1.x * p1.x + p1.y * p1.y) * (p2.y - p3.y) + (p2.x * p2.x + p2.y * p2.y) * (p3.y - p1.y) + (p3.x * p3.x + p3.y * p3.y) * (p1.y - p2.y)) / d;
    let uy = ((p1.x * p1.x + p1.y * p1.y) * (p3.x - p2.x) + (p2.x * p2.x + p2.y * p2.y) * (p1.x - p3.x) + (p3.x * p3.x + p3.y * p3.y) * (p2.x - p1.x)) / d;
    let center = Point { x: ux, y: uy };
    let radius = distance(&center, p1);
    Circle { center, radius }
}

// 计算边界点的中心
fn calculate_center(boundary: &[Point]) -> Point {
    let sum_x: f64 = boundary.iter().map(|p| p.x).sum();
    let sum_y: f64 = boundary.iter().map(|p| p.y).sum();
    let count = boundary.len() as f64;
    Point {
        x: sum_x / count,
        y: sum_y / count,
    }
}

#[test]
fn main() {
    // 示例数据
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.5, y: 0.5 },
        Point { x: 0.0, y: 1.0 },
    ];

    let boundary = Vec::new();

    let circle = welzl(&points, &boundary);
    println!("Center: ({}, {}), Radius: {}", circle.center.x, circle.center.y, circle.radius);
}