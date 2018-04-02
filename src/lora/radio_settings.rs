
#[derive(Clone, Copy)]
pub enum SpreadingFactor {
    SF6 = 6,
    SF7 = 7,
    SF8 = 8,
    SF9 = 9,
    SF10 = 10,
    SF11 = 11,
    SF12 = 12,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Bandwidth {
    _7_8kHz   = 0b0000,
    _10_4kHz  = 0b0001,
    _15_6kHz  = 0b0010,
    _20_8kHz  = 0b0011,
    _31_25kHz = 0b0100,
    _41_7kHz  = 0b0101,
    _62_5kHz  = 0b0110,
    _125kHz   = 0b0111,
    _250kHz   = 0b1000,
    _500kHz   = 0b1001,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ErrorCoding {
    _4_5 = 1,
    _4_6 = 2,
    _4_7 = 3,
    _4_8 = 4,
}

pub struct RadioSettings {
    pub f_rf: u32,
    pub bandwidth: Bandwidth,
    pub spreading_factor: SpreadingFactor,
    pub error_coding: ErrorCoding,
    pub crc: bool,
    pub implicit_header: bool,
    pub payload_length: u8,
    pub symbol_timeout: u16,
    pub low_datarate_optimize: bool,
    // TODO: AutoAgc, TxCont
}

pub struct PaSettings {
    pub pa_boost: bool,
    pub pa_dac_highpower: bool,
    pub power: i8,
}
