use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyModifier<T: Add<T, Output=T>+Sub<Output=T>+Clone> {
    Increment(T),
    Decrement(T),
    Set(T),
    NoChange
}

impl<T: Add<Output=T>+Sub<Output=T>+Clone> PropertyModifier<T> {
    pub fn value(&self, val: T) -> T {
        match *self {
            PropertyModifier::Increment(ref p) => val+p.clone(),
            PropertyModifier::Decrement(ref p) => val-p.clone(),
            PropertyModifier::Set(ref p) => p.clone(),
            PropertyModifier::NoChange => val
        }
    }
}