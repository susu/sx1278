

extern crate embedded_hal as hal;

mod mockdevice;
mod mockspi;
mod mockpin;


use std::cell::RefCell;
use std::rc::Rc;

use self::mockdevice::MockDevice;
use self::mockspi::MockSpi;
use self::mockpin::MockPin;

use sx1278::{SX1278, LoRa};

pub fn create_sx1278() -> (SX1278<MockSpi, MockPin, LoRa>, Rc<RefCell<MockDevice>>) {
    let device = Rc::new(RefCell::new(MockDevice::new()));
    let sx = SX1278::new_lora(MockSpi::new(Rc::clone(&device)), MockPin {set:false}).unwrap();
    println!("-- MARK: LORA driver created --");
    (sx, device)
}
