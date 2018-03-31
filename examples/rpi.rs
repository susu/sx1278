
extern crate linux_embedded_hal as linux_hal;
extern crate sx1278;

use linux_hal::{Spidev, Pin};
use linux_hal::spidev::SpidevOptions;
use linux_hal::sysfs_gpio::Direction;

use sx1278::SX1278;

fn main() {
    println!("Opening SPI...");
    let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI open error");
    let options = SpidevOptions::new()
        .max_speed_hz(1_000_000)
        .mode(linux_hal::spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).unwrap();

    let pin = Pin::new(25);
    pin.export().unwrap();
    while !pin.is_exported() {}
    pin.set_direction(Direction::Out).unwrap();
    pin.set_value(1).unwrap();

    let mut sx1278 = SX1278::new_lora(spi, pin).expect("SX1278 init error");
    println!("Version: 0x{:x}", sx1278.version().unwrap());

    // sx1278.

}
