
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

#[derive(Debug, PartialEq)]
pub enum PaError {
    PaBoostRequired,
    PowerOutOfRange,
    InvalidPaDacConfig,
}

#[derive(Debug, PartialEq)]
pub enum PaOutput {
    Rfo,
    PaBoost,
}

#[derive(Debug, PartialEq)]
pub enum PaDac {
    HighPower,
    NormalPower,
}

#[derive(Debug, PartialEq)]
pub struct PaSettings {
    pub output: PaOutput,
    pub pa_dac: PaDac,
    pub power: i8,
}

impl PaSettings {

    /// Returns the valid config where output is set to PA_BOOST
    ///
    /// Note: power must be: 14 < power <= 20
    /// If your circuit will run from battery, highly recommended to set OCP!
    ///
    /// Note: Ra-01 and Ra-02 (FIXME) must use this method (they are hardwired to PA_BOOST output)
    ///
    /// The implementation follows the logic of the reference driver from Semtech.
    pub fn with_pa_boost(power: i8) -> Result<PaSettings, PaError> {
        if power <= 14 || power > 20 {
            Err(PaError::PowerOutOfRange)
        } else {
            let pa_dac = if power > 17 {
                PaDac::HighPower
            } else {
                PaDac::NormalPower
            };

            Ok(PaSettings {
                output: PaOutput::PaBoost,
                pa_dac: pa_dac,
                power,
            })
        }
    }

    /// Returns the valid config where output is set to RFO
    ///
    /// Note: power must be: -1 <= power <= 14
    ///
    /// The implementation follows the logic of the reference driver from Semtech.
    pub fn with_rfo_output(power: i8) -> Result<PaSettings, PaError> {
        if power < -1 || power > 14 {
            Err(PaError::PowerOutOfRange)
        } else {
            Ok(PaSettings {
                output: PaOutput::Rfo,
                pa_dac: PaDac::NormalPower,
                power
            })
        }
    }
}
