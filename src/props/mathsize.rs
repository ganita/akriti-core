use std::f32;
use ::platform::DisplayMetrics;
use super::length::Length;

#[derive(Clone, PartialEq, Debug)]
pub enum MathSize {
    PX(f32),
    DP(f32),
    SP(f32),
    EM(f32),
    SMALL,
    NORMAL,
    BIG
}

const BIG_MATH_SIZE_MULTIPLIER: f32 = 1.2;
const SMALL_MATH_SIZE_MULTIPLIER: f32 = 0.8;

impl MathSize {
    pub fn get_math_size_px(&self, metrics: &DisplayMetrics, nominal_font_size_px: f32) -> f32 {
        match *self {
            MathSize::PX(px) => Length::PX(px).get_length_px(metrics, nominal_font_size_px),
            MathSize::DP(dp) => Length::DP(dp).get_length_px(metrics, nominal_font_size_px),
            MathSize::SP(sp) => Length::SP(sp).get_length_px(metrics, nominal_font_size_px),
            MathSize::EM(em) => Length::EM(em).get_length_px(metrics, nominal_font_size_px),
            MathSize::BIG => MathSize::NORMAL.get_math_size_px(metrics, nominal_font_size_px)
                *BIG_MATH_SIZE_MULTIPLIER,
            MathSize::NORMAL => nominal_font_size_px,
            MathSize::SMALL => MathSize::NORMAL.get_math_size_px(metrics, nominal_font_size_px)
                *SMALL_MATH_SIZE_MULTIPLIER,
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
            MathSize::PX(32.0).get_math_size_px(&metrics, 0.0),
            32.0
        )
    }

    #[test]
    fn test_dp_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            MathSize::DP(32.0).get_math_size_px(&metrics, 0.0),
            32.0*3.0
        )
    }

    #[test]
    fn test_sp_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            MathSize::SP(32.0).get_math_size_px(&metrics, 0.0),
            32.0*4.0
        )
    }

    #[test]
    fn test_em_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            MathSize::EM(2.0).get_math_size_px(&metrics, 64.0),
            64.0*2.0
        )
    }

    #[test]
    fn test_normal() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            MathSize::NORMAL.get_math_size_px(&metrics, 64.0),
            64.0
        )
    }

    #[test]
    fn test_big() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            MathSize::BIG.get_math_size_px(&metrics, 64.0),
            64.0*BIG_MATH_SIZE_MULTIPLIER
        )
    }

    #[test]
    fn test_small() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            MathSize::SMALL.get_math_size_px(&metrics, 64.0),
            64.0*SMALL_MATH_SIZE_MULTIPLIER
        )
    }

}