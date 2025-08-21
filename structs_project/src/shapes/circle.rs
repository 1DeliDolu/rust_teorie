// 🔵 Circle Struct - Daire
// Circle struct with methods and associated functions

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    // Associated function - Constructor
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    
    // Associated function - Create from diameter
    pub fn from_diameter(diameter: f64) -> Circle {
        Circle {
            radius: diameter / 2.0,
        }
    }
    
    // Method - Calculate area
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    // Method - Calculate perimeter (circumference)
    pub fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    // Method - Get diameter
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    
    // Method - Scale circle
    pub fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

// Second impl block - Comparison methods
impl Circle {
    // Method - Compare circles
    pub fn equals(&self, other: &Circle) -> bool {
        (self.radius - other.radius).abs() < f64::EPSILON
    }
    
    // Method - Check if this circle is larger
    pub fn is_larger_than(&self, other: &Circle) -> bool {
        self.area() > other.area()
    }
}

// Display trait implementation
impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle {{ radius: {} }}", self.radius)
    }
}