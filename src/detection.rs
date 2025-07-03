use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Detection {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Detection {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    // optional: getters
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.height }
    
    pub fn bbox(&self) -> (f32, f32, f32, f32) {
        (
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
        )
    }
    pub fn center(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, tol: f32) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn test_bbox_center() {
        let d = Detection { x: 0.5, y: 0.4, width: 0.2, height: 0.1 };
        assert_eq!(d.center(), (0.5, 0.4));
        assert!(approx_eq(d.bbox().0, 0.4, 1e-5));
        assert!(approx_eq(d.bbox().1, 0.35, 1e-5));
        assert!(approx_eq(d.bbox().2, 0.6, 1e-5));
        assert!(approx_eq(d.bbox().3, 0.45, 1e-5));
    }

}

