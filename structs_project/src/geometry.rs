// 🧮 Geometry Utilities - Geometrik Hesaplayıcılar
// Bu modül genel geometrik hesaplamaları içerir

pub struct ShapeCalculator;

impl ShapeCalculator {
    // Associated function - Calculate total area of rectangles
    pub fn total_rectangle_area(rectangles: &[crate::shapes::Rectangle]) -> f64 {
        rectangles.iter().map(|rect| rect.area()).sum()
    }
    
    // Associated function - Calculate total area of circles
    pub fn total_circle_area(circles: &[crate::shapes::Circle]) -> f64 {
        circles.iter().map(|circle| circle.area()).sum()
    }
    
    // Associated function - Find largest rectangle by area
    pub fn largest_rectangle(rectangles: &[crate::shapes::Rectangle]) -> Option<&crate::shapes::Rectangle> {
        rectangles.iter().max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
    }
    
    // Associated function - Convert degrees to radians
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }
    
    // Associated function - Convert radians to degrees
    pub fn radians_to_degrees(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }
}