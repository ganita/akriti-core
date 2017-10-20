use super::Length;

#[derive(Debug, Clone)]
pub struct FrameSpacing {
    left: Length,
    right: Length
}

impl FrameSpacing {
    pub fn new(left: Length, right: Length) -> FrameSpacing {
        FrameSpacing {
            left, right
        }
    }

    pub fn left(&self) -> &Length {
        &self.left
    }

    pub fn right(&self) -> &Length {
        &self.right
    }
}