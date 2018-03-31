extern crate sx1278;
extern crate embedded_hal as hal;

use hal::blocking::spi;
use std::io;
use std::sync::mpsc::{Receiver, Sender};

use sx1278::{SX1278, LoRa};

struct DeviceMock {
    registry: [u8; 128],
    fifo: [u8; 256],
    receiver: Receiver<Vec<u8>>,
}

struct MockSpi {
    sender: Sender<Vec<u8>>
}

impl MockSpi {
    fn new() -> MockSpi {
    }
}

impl spi::Transfer<u8> for MockSpi {
    type Error = io::Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> io::Result<&'b [u8]> {
        // let tx = buffer.to_owned();
        Ok(buffer)
    }
}

struct NoopPin {
    set: bool
}

impl hal::digital::OutputPin for NoopPin {
    fn is_low(&self) -> bool { !self.set }
    fn is_high(&self) -> bool { self.set }
    fn set_low(&mut self) { self.set = false; }
    fn set_high(&mut self) { self.set = true; }
}

struct Fixture<'a> {
    spi: &'a MockSpi,
    sx: SX1278<MockSpi, NoopPin, LoRa>
}

impl<'a> Fixture<'a> {
    fn new() -> Fixture<'a> {
        let spi = MockSpi::new();

        Fixture {
            spi: &spi,
            sx: SX1278::new_lora(spi, NoopPin { set: false }).unwrap(),
        }
    }
}

#[test]
fn read_initialize() {
    let mut f = Fixture::new();
}
