
static mut MOCK_DEVICE: MockDevice = MockDevice {
    registry: [0; 128],
    fifo: [0; 256],
};

pub fn get_mock_device() -> &'static mut MockDevice {
    unsafe { &mut MOCK_DEVICE }
}


pub struct MockDevice {
    pub registry: [u8; 128],
    pub fifo: [u8; 256],
}

impl MockDevice {
    pub fn reset(&mut self) {
        self.fifo = [0; 256];
        self.registry = [0; 128];
        // OpMode:
        self.registry[0x01] = 0b0000_1001; // FSK mode on, noshared, lowfreq regs, standby

        // Frf:
        self.registry[0x06] = 0x6c;
        self.registry[0x07] = 0x80;
        self.registry[0x08] = 0x00;

        // version
        self.registry[0x42] = 0x12;
    }
}

