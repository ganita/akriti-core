#[macro_use] extern crate akriti_macros;

pub extern crate akriti_constants;
pub use akriti_constants as constants;

pub mod paint;
pub mod props;
pub mod platform;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert!(true);
    }
}