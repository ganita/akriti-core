
#[derive(Debug, Clone)]
pub struct Rect {
    width: f32,
    height: f32
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Rect {
        assert!(width >= 0.0, "Width of rectangle should be greater than zero");
        assert!(height >= 0.0, "Height of rectangle should be greater than zero");
        Rect {
            width,
            height
        }
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}