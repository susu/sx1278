
use std::io;
use utils::hal::blocking::spi;

use utils::mockdevice::get_mock_device;

pub struct MockSpi;

impl MockSpi {
    pub fn new() -> MockSpi {
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
