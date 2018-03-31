extern crate sx1278;
extern crate embedded_hal as hal;

use hal::blocking::spi;
use std::io;

use sx1278::{SX1278, LoRa};


static mut MOCK_DEVICE: MockDevice = MockDevice {
    registry: [0; 128],
    fifo: [0; 256],
};


fn get_mock_device() -> &'static mut MockDevice {
    unsafe { &mut MOCK_DEVICE }
}

struct MockDevice {
    registry: [u8; 128],
    fifo: [u8; 256],
}

impl MockDevice {
    fn reset(&mut self) {
        self.fifo = [0; 256];
        self.registry = [0; 128];
        // OpMode:
        self.registry[0x01] = 0b0000_1001; // FSK mode on, noshared, lowfreq regs, standby
        // Frf:
        self.registry[0x06] = 0x6c;
        self.registry[0x07] = 0x80;
        self.registry[0x08] = 0x00;
    }
}

struct MockSpi;

impl MockSpi {
    fn new() -> MockSpi {
        get_mock_device().reset();
        println!("MockSpi created, mockdevice reset.");
        MockSpi { }
    }
}

impl spi::Transfer<u8> for MockSpi {
    type Error = io::Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> io::Result<&'b [u8]> {
        // let tx = buffer.to_owned();
        assert!(buffer.len() == 2, "Only implemented for single byte SPI xfers");
        println!("SPI XFER: <- in {:?}", buffer);
        const WRITE_MASK: u8 = 0b1000_0000;
        if buffer[0] & WRITE_MASK == WRITE_MASK {
            let addr = buffer[0] & !WRITE_MASK;
            println!("             WRITE: reg 0x{:02x}, {:08b}", addr, buffer[1]);
            get_mock_device().registry[addr as usize] = buffer[1];
            buffer[0] = 0; // Usually some garbage, the byte while Master sent the address.
            buffer[1] = 0; // Usually the previous value of the registry, but not documented, so zero
        } else {
            let addr = buffer[0];
            buffer[1] = get_mock_device().registry[addr as usize];
            println!("             READ: reg 0x{:02x} -> {:08b}", addr, buffer[1]);
        }
        println!("          -> out: {:?}", buffer);
        Ok(buffer)
    }
}

struct NoopPin { set: bool }

impl hal::digital::OutputPin for NoopPin {
    fn is_low(&self) -> bool { !self.set }
    fn is_high(&self) -> bool { self.set }
    fn set_low(&mut self) { self.set = false; }
    fn set_high(&mut self) { self.set = true; }
}

fn create_sx1278() -> SX1278<MockSpi, NoopPin, LoRa> {
    SX1278::new_lora(MockSpi::new(), NoopPin {set:false}).unwrap()
}

fn registry_value(addr: u8) -> u8 {
    get_mock_device().registry[addr as usize]
}

#[test]
fn upon_creation_it_should_be_in_sleep_and_lora_mode() {
    let mut sx = create_sx1278();

    // TODO some bit eq with nice debug output would be great
    assert_eq!(registry_value(0x01) & 0b1000_0111, 0b1000_0000);
    //                                          lora ^     ^^^ sleep
}
