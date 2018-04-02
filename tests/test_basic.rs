extern crate sx1278;
mod utils;

use utils::create_sx1278;

#[test]
fn upon_creation_it_should_be_in_sleep_and_lora_mode() {
    let (_sx, device) = create_sx1278();

    // TODO some bit eq with nice debug output would be great
    assert_eq!(device.borrow().registry_value(0x01) & 0b1000_0111, 0b1000_0000);
    //                                          lora ^     ^^^ sleep
}

#[test]
fn it_should_return_version() {
    let (mut sx, _) = create_sx1278();
    assert_eq!(sx.version().unwrap(), 0x12);
}
