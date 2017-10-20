use std::f32;
use ::platform::DisplayMetrics;
use super::length::Length;

#[derive(Clone, PartialEq, Debug)]
pub enum LineThickness {
    PX(f32),
    DP(f32),
    SP(f32),
    EM(f32),
    THIN,
    THICK,
    MEDIUM
}

const THICK_LINE_THICKNESS_MULTIPLIER: f32 = 1.2;
const THIN_LINE_THICKNESS_MULTIPLIER: f32 = 0.8;

impl LineThickness {
    pub fn get_thickness_px(&self, metrics: &DisplayMetrics, font_size_px: f32, nominal_rule_thickness: f32) -> f32 {
        match *self {
            LineThickness::PX(px) => Length::PX(px).get_length_px(metrics, font_size_px),
            LineThickness::DP(dp) => Length::DP(dp).get_length_px(metrics, font_size_px),
            LineThickness::SP(sp) => Length::SP(sp).get_length_px(metrics, font_size_px),
            LineThickness::EM(em) => Length::EM(em).get_length_px(metrics, font_size_px),
            LineThickness::THICK => LineThickness::MEDIUM.get_thickness_px(metrics, font_size_px, nominal_rule_thickness)
                *THICK_LINE_THICKNESS_MULTIPLIER,
            LineThickness::MEDIUM => nominal_rule_thickness,
            LineThickness::THIN => LineThickness::MEDIUM.get_thickness_px(metrics, font_size_px, nominal_rule_thickness)
                *THIN_LINE_THICKNESS_MULTIPLIER,
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    fn get_test_display_metrics() -> DisplayMetrics {
        DisplayMetrics::new(3.0, 4.0)
    }

    #[test]
    fn test_px_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            LineThickness::PX(32.0).get_thickness_px(&metrics, 0.0, 0.0),
            32.0
        )
    }

    #[test]
    fn test_dp_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            LineThickness::DP(32.0).get_thickness_px(&metrics, 0.0, 0.0),
            32.0*3.0
        )
    }

    #[test]
    fn test_sp_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            LineThickness::SP(32.0).get_thickness_px(&metrics, 0.0, 0.0),
            32.0*4.0
        )
    }

    #[test]
    fn test_em_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            LineThickness::EM(2.0).get_thickness_px(&metrics, 64.0, 0.0),
            64.0*2.0
        )
    }

    #[test]
    fn test_medium() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            LineThickness::MEDIUM.get_thickness_px(&metrics, 64.0, 2.0),
            2.0
        )
    }

    #[test]
    fn test_thick() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            LineThickness::THICK.get_thickness_px(&metrics, 64.0, 2.0),
            2.0*THICK_LINE_THICKNESS_MULTIPLIER
        )
    }

    #[test]
    fn test_thin() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            LineThickness::THIN.get_thickness_px(&metrics, 64.0, 2.0),
            2.0*THIN_LINE_THICKNESS_MULTIPLIER
        )
    }

}