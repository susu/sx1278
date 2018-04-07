#![no_std]
//! Platform agnostic driver for SX1278 using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!
//!


extern crate embedded_hal as hal;

use core::marker::PhantomData;

pub enum LoRa {}
pub enum FskOok {}

pub mod lora;

/// Represents an SX1278 device
///
/// Implemented only for SPI communications for now, no DIOx things.
///
/// RFMODE: LoRa or FskOok (implemented only for LoRa)
pub struct SX1278<SPI, NSS, RFMODE> {
    spi: SPI,
    nss: NSS,
    _reg_mode: PhantomData<RFMODE>
}

use hal::blocking::spi;
use hal::digital::OutputPin;

impl<E, SPI, NSS> SX1278<SPI, NSS, ()>
where
    SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E>,
    NSS: OutputPin,
{
    /// Creates a new driver instance and ensures that it is in LoRa RF / SLEEP opmode.
    pub fn new_lora(spi: SPI, nss: NSS) -> Result<SX1278<SPI, NSS, LoRa>, E> {
        SX1278::new(spi, nss)
    }

    // pub fn new_fskook(spi: SPI, nss: NSS) -> SX1278<SPI, NSS, FskOok> {}
}

