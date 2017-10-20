
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayMetrics {
    density: f32,
    scaled_density: f32
}

impl DisplayMetrics {
    pub fn new(density: f32, scaled_density: f32) -> DisplayMetrics {
        DisplayMetrics { density, scaled_density }
    }

    pub fn dp_to_px(&self, dp: f32) -> f32 {
        self.density*dp
    }

    pub fn sp_to_px(&self, sp: f32) -> f32 {
        self.scaled_density*sp
    }

    pub fn px_to_dp(&self, px: f32) -> f32 {
        px/self.density
    }

    pub fn px_to_sp(&self, px: f32) -> f32 {
        px/self.scaled_density
    }
}