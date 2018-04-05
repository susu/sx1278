
pub struct MockDevice {
    pub registry: [u8; 128],
    pub fifo: [u8; 256],
}

impl MockDevice {
    pub fn new() -> MockDevice {
        let mut device = MockDevice {
            registry: [0; 128],
            fifo: [0; 256],
        };
        device.reset();
        device
    }

    pub fn reset(&mut self) {
        self.fifo = [0; 256];
        self.set_default_registry_values();
    }

    pub fn registry_value(&self, addr: u8) -> u8 {
        self.registry[addr as usize]
    }

    fn set_default_registry_values(&mut self) {
        self.registry = [0; 128];
        // NOTE: missed values are zeros or forgotten
        // OpMode
        self.registry[0x01] = 0b0000_1001; // FSK mode on, noshared, lowfreq regs, standby

        // Frf (434.0 MHz)
        self.registry[0x06] = 0x6c;
        self.registry[0x07] = 0x80;
        self.registry[0x08] = 0x00;

        //  PaConfig
        self.registry[0x09] = 0b0100_1111; // RFO, MaxPower:4, Output:15
        // PaRamp
        self.registry[0x0a] = 0x09; // 40 usec
        // OCP
        self.registry[0x0b] = 0b00_1_01011; // (unused), OcpOn, OcpTrim (100mA)
        // LNA
        self.registry[0x0c] = 0b0010_0000;

        // TxBaseAddr
        self.registry[0x0e] = 0x80;

        // ModemStat
        self.registry[0x18] = 0b0001_0000;

        // ModemConfig1
        self.registry[0x1d] = 0b0111_001_0; // BW, ErrorCoding, ImplicitHeader
        self.registry[0x1e] = 0b0111_0_0_00; // SF7, TxConf, CRC, Symbtimeout MSB
        // Symbol Timeout LSB
        self.registry[0x1f] = 0x64;

        // Preamble Length LSB
        self.registry[0x21] = 0x08;
        // Payload Length
        self.registry[0x22] = 0x01;
        // Max. payload length
        self.registry[0x23] = 0xff;

        // Detect Optimize
        self.registry[0x31] = 0b11000_011; // (reserved), 0x03 (SF7 - SF12)

        // InvertIQ
        self.registry[0x33] = 0b0_0_100111; // (reserved), no InvertIQ, (reserved)

        // DetectionThreshold
        self.registry[0x37] = 0x0a; // SF7 - SF12

        // SyncWord
        self.registry[0x39] = 0x12;

        // version
        self.registry[0x42] = 0x12;

        // PaDac
        self.registry[0x4d] = (0x10 << 3) | 0x04
    }
}

