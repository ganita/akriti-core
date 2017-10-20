use std::f32;
use ::platform::DisplayMetrics;
use ::constants::SpaceLevel;

#[derive(Clone, PartialEq, Debug)]
pub enum Length {
    PX(f32),
    DP(f32),
    SP(f32),
    EM(f32),
    EX(f32),
    SpaceLevel(SpaceLevel),
    Infinity,
    Auto
}

impl Length {
    pub fn get_length_px(&self, metrics: &DisplayMetrics, font_size_px: f32) -> f32 {
        match *self {
            Length::PX(px) => px,
            Length::DP(dp) => metrics.dp_to_px(dp),
            Length::SP(sp) => metrics.sp_to_px(sp),
            Length::EM(em) => em*font_size_px,
            Length::SpaceLevel(ref level) => level.em()*font_size_px,
            Length::Infinity => f32::INFINITY,
            Length::Auto => f32::NAN,
            Length::EX(_) => unimplemented!()
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
            Length::PX(32.0).get_length_px(&metrics, 0.0),
            32.0
        )
    }

    #[test]
    fn test_dp_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            Length::DP(32.0).get_length_px(&metrics, 0.0),
            32.0*3.0
        )
    }

    #[test]
    fn test_sp_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            Length::SP(32.0).get_length_px(&metrics, 0.0),
            32.0*4.0
        )
    }

    #[test]
    fn test_em_unit() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            Length::EM(2.0).get_length_px(&metrics, 64.0),
            64.0*2.0
        )
    }

    #[test]
    fn test_infinity() {
        let metrics = get_test_display_metrics();
        assert_eq!(
            Length::Infinity.get_length_px(&metrics, 0.0),
            f32::INFINITY
        )
    }

    #[test]
    fn test_auto() {
        let metrics = get_test_display_metrics();
        assert!(
            f32::is_nan(Length::Auto.get_length_px(&metrics, 0.0))
        )

    }

}