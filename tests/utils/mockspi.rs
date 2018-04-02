
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
        // let tx = buffer.to_owned();
        assert!(buffer.len() == 2, "Only implemented for single byte SPI xfers");
        println!("SPI XFER: <- in {:?}", buffer);
        const WRITE_MASK: u8 = 0b1000_0000;
        if buffer[0] & WRITE_MASK == WRITE_MASK {
            let addr = buffer[0] & !WRITE_MASK;
            println!("             WRITE: reg 0x{:02x}, {:08b}", addr, buffer[1]);
            self.device.borrow_mut().registry[addr as usize] = buffer[1];
            buffer[0] = 0; // Usually some garbage, the byte while Master sent the address.
            buffer[1] = 0; // Usually the previous value of the registry, but not documented, so zero
        } else {
            let addr = buffer[0];
            buffer[1] = self.device.borrow_mut().registry[addr as usize];
            println!("             READ: reg 0x{:02x} -> {:08b}", addr, buffer[1]);
        }
        println!("          -> out: {:?}", buffer);
        Ok(buffer)
    }
}
