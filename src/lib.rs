
extern crate embedded_hal as hal;

mod register;
mod mode;

use register::Register;
use mode::Mode;

pub struct SX1278<SPI, NSS> {
    spi: SPI,
    nss: NSS,
}

use hal::blocking::spi;
use hal::digital::OutputPin;

impl<E, SPI, NSS> SX1278<SPI, NSS>
where
    SPI: spi::Transfer<u8, Error = E>,
    NSS: OutputPin,
{
    pub fn new(spi: SPI, nss: NSS) -> Self {
        SX1278 { spi, nss }
    }

    pub fn version(&mut self) -> Result<u8, E> {
        self.read(Register::Version)
    }

    pub fn mode(&mut self) -> Result<Mode, E> {
        self.read(Register::OpMode).map(|opmode| Mode::from_opmode(opmode))
    }

    pub fn set_mode(&mut self, new_mode: &Mode) -> Result<(), E> {
        let mut opmode = self.read(Register::OpMode)?;
        opmode = (opmode & 0b1111_1000) | (*new_mode as u8);
        self.write(Register::OpMode, opmode)
    }

    pub fn set_F_rf(&mut self, F_rf: u32) -> Result<(), E> {
        let msb = (F_rf >> 16) & 0xff;
        let mid = (F_rf >> 8) & 0xff;
        let lsb = F_rf & 0xff;
        self.write(Register::FreqMsb, msb as u8)?; // TODO write burst
        self.write(Register::FreqMid, mid as u8)?;
        self.write(Register::FreqLsb, lsb as u8)
    }

    // bus
    fn read(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer = [0x80 | reg.addr(), 0];
        self.nss.set_low();
        let buffer = self.spi.transfer(&mut buffer)?;
        self.nss.set_high();
        Ok(buffer[1])
    }

    fn write(&mut self, reg: Register, value: u8) -> Result<(), E> {
        let mut buffer = [0x00 | reg.addr(), value];
        self.nss.set_low();
        self.spi.transfer(&mut buffer)?;
        self.nss.set_high();
        Ok(())
    }
}
