use std::f32;
use ::platform::Context;
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
    pub fn get_length_du(&self, context: &Context, font_size_du: f32) -> f32 {
        match *self {
            Length::PX(px) => context.platform().px_to_du(px),
            Length::DP(dp) => context.platform().dp_to_du(dp),
            Length::SP(sp) => context.platform().sp_to_du(sp),
            Length::EM(em) => em*font_size_du,
            Length::SpaceLevel(ref level) => level.em()*font_size_du,
            Length::Infinity => f32::INFINITY,
            Length::Auto => f32::NAN,
            Length::EX(_) => unimplemented!()
        }
    }
}