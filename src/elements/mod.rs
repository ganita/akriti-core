mod presentation;               pub use self::presentation::*;

mod token;                      pub use self::token::*;
mod general_layout;             pub use self::general_layout::*;

use ::platform::Context;
use ::draw::Drawable;

pub trait Element {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a>;
}

pub trait ConcreteElement<'a, T: Drawable + 'a> {
    fn layout(&'a self, context: &Context) -> T;
}

pub trait ElementGroup : Element {
    fn children(&self) -> &[Box<Element>];
}