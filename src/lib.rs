pub extern crate humansize;
#[macro_use]
extern crate lazy_static;

pub mod bool;
pub mod numbers;

pub use humansize as sizes;
pub use numbers::HumanizeNumbers;
pub use bool::ToBool;
