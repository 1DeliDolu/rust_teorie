// 📐 Rectangle Struct - Dikdörtgen
// Struct definition, methods, ve associated functions

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// İlk impl block - Basic methods
impl Rectangle {
    // Associated function - Constructor
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Associated function - Factory method for square
    pub fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Method - Calculate area (&self)
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // Method - Calculate perimeter (&self)
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    // Method - Scale rectangle (&mut self)
    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Method - Check if square
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

// İkinci impl block - Comparison methods
impl Rectangle {
    // Method - Compare with another rectangle
    pub fn equals(&self, other: &Rectangle) -> bool {
        (self.width - other.width).abs() < f64::EPSILON 
            && (self.height - other.height).abs() < f64::EPSILON
    }
    
    // Method - Check if can hold another rectangle
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Display trait implementation
impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle {{ width: {}, height: {} }}", self.width, self.height)
    }
}