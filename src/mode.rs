
#[derive(Clone, Copy)]
pub enum Mode {
    Sleep = 0x00,
    Standby = 0x01,
    FsTx = 0x02,
    Tx = 0x03,
    FsRx = 0x04,
    RxCont = 0x05,
    RxSingle = 0x06,
    Cad = 0x07,
}

impl Mode {
    pub fn from_opmode(opmode: u8) -> Mode {
        match opmode & 0b111 {
            0x00 => Mode::Sleep,
            0x01 => Mode::Standby,
            0x02 => Mode::FsTx,
            0x03 => Mode::Tx,
            0x04 => Mode::FsRx,
            0x05 => Mode::RxCont,
            0x06 => Mode::RxSingle,
            0x07 => Mode::Cad,
            _ => panic!("unreachable")
        }
    }
}
