
pub enum SpreadingFactor {
    SF6 = 6,
    SF7 = 7,
    SF8 = 8,
    SF9 = 9,
    SF10 = 10,
    SF11 = 11,
    SF12 = 12,
}

pub enum Bandwidth {
    _7_8kHz   = 0b0000,
    _10_4kHz  = 0b0001,
    _15_6kHz  = 0b0010,
    _20_8kHz  = 0b0100,
    _31_25kHz = 0b0101,
    _62_5kHz  = 0b0110,
    _125kHz   = 0b0111,
    _250kHz   = 0b1000,
    _500kHz   = 0b1001,
}

pub enum ErrorCoding {
    _4_5 = 1,
    _4_6 = 2,
    _4_7 = 3,
    _4_8 = 4,
}

struct RadioSettings {
    F_rf: u32,
    bandwidth: Bandwidth,
    spreading_factor: SpreadingFactor,
    error_coding: ErrorCoding,
    crc: bool,
    implicit_header: bool,
    payload_length: u8,
    symbol_timeout: u16,
    low_datarate_optimize: bool,
}

struct PaSettings {
    pa_boost: bool,
    pa_dac_highpower: bool,
    power: i8,
}
