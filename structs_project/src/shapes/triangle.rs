// 🔺 Triangle Struct - Üçgen
// Triangle struct with methods

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub base: f64,
    pub height: f64,
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

impl Triangle {
    // Associated function - Constructor for general triangle
    pub fn new(side_a: f64, side_b: f64, side_c: f64) -> Option<Triangle> {
        // Check triangle inequality
        if side_a + side_b > side_c && side_a + side_c > side_b && side_b + side_c > side_a {
            // Calculate base and height (using Heron's formula concepts)
            let s = (side_a + side_b + side_c) / 2.0;
            let area = (s * (s - side_a) * (s - side_b) * (s - side_c)).sqrt();
            let height = 2.0 * area / side_a; // Use side_a as base
            
            Some(Triangle {
                base: side_a,
                height,
                side_a,
                side_b,
                side_c,
            })
        } else {
            None // Invalid triangle
        }
    }
    
    // Associated function - Right triangle constructor
    pub fn right_triangle(base: f64, height: f64) -> Triangle {
        let hypotenuse = (base * base + height * height).sqrt();
        Triangle {
            base,
            height,
            side_a: base,
            side_b: height,
            side_c: hypotenuse,
        }
    }
    
    // Method - Calculate area
    pub fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    
    // Method - Calculate perimeter
    pub fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
    
    // Method - Check if right triangle
    pub fn is_right_triangle(&self) -> bool {
        let sides = [self.side_a, self.side_b, self.side_c];
        let mut sides = sides;
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let a_sq = sides[0] * sides[0];
        let b_sq = sides[1] * sides[1];
        let c_sq = sides[2] * sides[2];
        
        (a_sq + b_sq - c_sq).abs() < f64::EPSILON
    }
}

// Display trait implementation
impl std::fmt::Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle {{ sides: {}, {}, {} }}", self.side_a, self.side_b, self.side_c)
    }
}