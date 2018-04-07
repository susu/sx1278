
use std::io;
use utils::hal::blocking::spi;

use std::rc::Rc;
use std::cell::RefCell;
use utils::mockdevice::MockDevice;

pub struct MockSpi {
    device: Rc<RefCell<MockDevice>>,
}

impl MockSpi {
    pub fn new(device: Rc<RefCell<MockDevice>>) -> MockSpi {
        println!("MockSpi created, mockdevice reset.");
        MockSpi { device }
    }
}

impl spi::Transfer<u8> for MockSpi {
    type Error = io::Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> io::Result<&'b [u8]> {
        assert!(buffer.len() == 2, "Only implemented for single byte SPI xfers");

        print!("  SPI XFER: <- in [");

        for byte in buffer.iter_mut() {
            *byte = self.device.borrow_mut().process_byte(*byte);
        }

        println!("]");

        Ok(buffer)
    }
}

impl spi::Write<u8> for MockSpi {
    type Error = io::Error;

    fn write(&mut self, buffer: &[u8]) -> io::Result<()> {
        print!("  SPI WRITE: <- in [");

        for byte in buffer.iter() {
            self.device.borrow_mut().process_byte(*byte);
        }

        println!("]");
        Ok(())
    }
}
