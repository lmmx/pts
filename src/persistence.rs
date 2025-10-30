use facet::Facet;
use std::fs;

const POINTS_FILE: &str = "points.json";

#[derive(Facet, Clone)]
#[repr(u8)]
pub enum PointShape {
    Circle,
    Square,
}

#[derive(Facet, Clone)]
pub struct Point {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub shape: PointShape,
}

#[derive(Facet, Clone)]
struct Points {
    points: Vec<Point>,
}

pub fn load_points() -> Vec<Point> {
    let _ = fs::remove_file(POINTS_FILE);
    vec![
        Point { id: 1, x: 100.0, y: 100.0, shape: PointShape::Circle },
        Point { id: 2, x: 200.0, y: 200.0, shape: PointShape::Circle },
        Point { id: 3, x: 300.0, y: 150.0, shape: PointShape::Square },
    ]
}

pub fn save_points(points: &[Point]) {
    let wrapped = Points { points: points.to_vec() };
    let json = facet_json::to_string(&wrapped);
    let _ = fs::write(POINTS_FILE, json);
}
