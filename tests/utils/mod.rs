
extern crate embedded_hal as hal;

pub mod mockdevice;
pub mod mockspi;
pub mod mockpin;

pub use self::mockpin::MockPin;

