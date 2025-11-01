//! Point data structures and JSON serialization.

use facet::Facet;
use std::fs;

const POINTS_FILE: &str = "points.json";

#[derive(Copy, Clone, Facet, PartialEq)]
#[repr(u8)]
pub enum PointShape {
    Circle,
    Square,
    Diamond,
    Semicircle,
}

#[derive(Facet, Clone)]
pub struct Point {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub shape: PointShape,
    #[facet(default = 0.0)]
    pub rotation: f32, // in radians
}

#[derive(Facet, Clone)]
struct Points {
    points: Vec<Point>,
}

#[must_use]
pub fn load_points() -> Vec<Point> {
    let _ = fs::remove_file(POINTS_FILE);
    vec![
        Point {
            id: 1,
            x: 400.0,
            y: 200.0,
            shape: PointShape::Circle,
            rotation: 0.0,
        },
        Point {
            id: 2,
            x: 500.0,
            y: 300.0,
            shape: PointShape::Square,
            rotation: 0.0,
        },
        Point {
            id: 3,
            x: 600.0,
            y: 400.0,
            shape: PointShape::Diamond,
            rotation: 0.0,
        },
        Point {
            id: 4,
            x: 700.0,
            y: 500.0,
            shape: PointShape::Semicircle,
            rotation: 0.0,
        },
    ]
}

pub fn save_points(points: &[Point]) {
    let wrapped = Points {
        points: points.to_vec(),
    };
    let json = facet_json::to_string(&wrapped);
    let _ = fs::write(POINTS_FILE, json);
}
