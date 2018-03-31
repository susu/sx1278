
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Register {
    Fifo             = 0x00,
    OpMode           = 0x01,
    FreqMsb          = 0x06,
    FreqMid          = 0x07,
    FreqLsb          = 0x08,
    PaConfig         = 0x09,
    PaRamp           = 0x0a,
    Ocp              = 0x0b,
    Lna              = 0x0c,
    FifoAddrPtr      = 0x0d,
    FifoTxBaseAddr   = 0x0e,
    FifoRxBaseAddr   = 0x0f,
    FifoRxCurrAddr   = 0x10,
    IrqFlagsMask     = 0x11,
    IrqFlags         = 0x12,
    RxNbBytes        = 0x13,
    RxHeaderCountMsb = 0x14,
    RxHeaderConutLsb = 0x15,
    RxPacketCountMsb = 0x16,
    RxPacketCountLsb = 0x17,
    ModemStat        = 0x18,
    PacketSnr        = 0x19,
    PacketRssi       = 0x1a,
    Rssi             = 0x1b,
    HopChannel       = 0x1c,
    ModemConfig1     = 0x1d,
    ModemConfig2     = 0x1e,
    SymbTimeoutLsb   = 0x1f,
    PreambleMsb      = 0x20,
    PreambleLsb      = 0x21,
    PayloadLength    = 0x22,
    MaxPayloadLength = 0x23,
    HopPeriod        = 0x24,
    FifoRxByteAddr   = 0x25,
    ModemConfig3     = 0x26,
    PpmCorrection    = 0x27,
    FreqErrorMsb     = 0x28,
    FreqErrorMid     = 0x29,
    FreqErrorLsb     = 0x2A,
    RssiWideband     = 0x2c,
    DetectOptimize   = 0x31,
    InvertIq         = 0x33,
    DetectionThresh  = 0x34,
    SyncWord         = 0x39,
    DioMapping1      = 0x40,
    Version          = 0x42,
    PaDac            = 0x4d,
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}
