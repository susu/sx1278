extern crate sx1278;

mod utils;

use utils::mockdevice::get_mock_device;
use utils::mockspi::MockSpi;
use utils::MockPin;

use sx1278::{SX1278, LoRa};

fn create_sx1278() -> SX1278<MockSpi, MockPin, LoRa> {
    let sx = SX1278::new_lora(MockSpi::new(), MockPin {set:false}).unwrap();
    println!("-- MARK: LORA driver created --");
    sx
}

fn registry_value(addr: u8) -> u8 {
    get_mock_device().registry[addr as usize]
}

#[test]
fn upon_creation_it_should_be_in_sleep_and_lora_mode() {
    let _sx = create_sx1278();

    // TODO some bit eq with nice debug output would be great
    assert_eq!(registry_value(0x01) & 0b1000_0111, 0b1000_0000);
    //                                          lora ^     ^^^ sleep
}

#[test]
fn it_should_return_version() {
    let mut sx = create_sx1278();
    assert_eq!(sx.version().unwrap(), 0x12);
}
