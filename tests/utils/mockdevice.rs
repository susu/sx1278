
#[derive(Debug)]
enum CurrentOp {
    FifoRead(usize),
    FifoWrite(usize),

    BurstRead(usize),
    BurstWrite(usize),
    // Single byte is a special case of the above.
}

pub struct MockDevice {
    pub registry: [u8; 128],
    pub fifo: [u8; 256],
    pub chip_selected: bool,
    current_op: Option<CurrentOp>,
    next_byte_to_read: u8,
}

impl MockDevice {
    pub fn new() -> MockDevice {
        let mut device = MockDevice {
            registry: [0; 128],
            fifo: [0; 256],
            chip_selected: false,
            current_op: None,
            next_byte_to_read: 0,
        };
        device.reset();
        device
    }

    pub fn reset(&mut self) {
        self.fifo = [0; 256];
        self.set_default_registry_values();
        self.chip_selected = false;
        self.current_op = None;
        self.next_byte_to_read = 0;
    }

    pub fn registry_value(&self, addr: u8) -> u8 {
        self.registry[addr as usize]
    }

    pub fn set_fifo_value(&mut self, addr: u8, value: u8) {
        self.fifo[addr as usize] = value;
    }

    pub fn process_byte(&mut self, byte: u8) -> u8 {
        assert!(self.chip_selected, "Got data without chip-select! {:x}", byte);
        print!("{:02x} ", byte);
        let ret_byte = self.next_byte_to_read;

        self.next_byte_to_read = match self.current_op {
            None => {
                let (op, next_byte) = self.new_op(byte);
                self.current_op = Some(op);
                next_byte
            },
            Some(CurrentOp::FifoRead(last_fifo_addr)) => {
                self.registry[0x0d] += 1; // TODO check if the chip really increments this ptr
                self.current_op = Some(CurrentOp::FifoRead(last_fifo_addr + 1));
                self.fifo[last_fifo_addr + 1]
            },
            Some(CurrentOp::FifoWrite(fifo_addr)) => {
                self.current_op = Some(CurrentOp::FifoWrite(fifo_addr + 1));
                self.fifo[fifo_addr] = byte;
                0u8
            },
            Some(CurrentOp::BurstRead(last_addr)) => {
                self.current_op = Some(CurrentOp::BurstRead(last_addr + 1));
                self.registry[last_addr + 1]
            },
            Some(CurrentOp::BurstWrite(addr)) => {
                self.current_op = Some(CurrentOp::BurstWrite(addr + 1));
                self.registry[addr] = byte;
                0u8
            },
        };

        ret_byte
    }

    fn new_op(&self, byte: u8) -> (CurrentOp, u8) {
        match Op::from(byte) {
            Op::Write(addr) if addr == 0x00 => {
                print!("FifoWrite");
                (CurrentOp::FifoWrite(self.registry_value(0x0d) as usize), 0)
            },
            Op::Read(addr) if addr == 0x00 => {
                let fifo_ptr = self.registry_value(0x0d) as usize;
                print!("FifoRead({:02x})", fifo_ptr);
                (CurrentOp::FifoRead(fifo_ptr), self.fifo[fifo_ptr])
            },
            Op::Write(addr) => {
                print!("W({:02x}) ", addr);
                (CurrentOp::BurstWrite(addr), 0)
            },
            Op::Read(addr) => {
                print!("R({:02x}) ", addr);
                (CurrentOp::BurstRead(addr), self.registry[addr])
            },
        }
    }

    pub fn select_chip(&mut self) {
        assert!(!self.chip_selected, "Double chip-select!");
        self.chip_selected = true;
        println!("<< select <<");
    }

    pub fn deselect_chip(&mut self) {
        assert!(self.chip_selected, "Double chip-de-select!");
        self.chip_selected = false;
        self.current_op = None;
        println!(">> deselect >>");
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

enum Op {
    Read(usize),
    Write(usize),
}

impl Op {
    fn from(byte: u8) -> Op {
        const WRITE_MASK: u8 = 0b1000_0000;
        if byte & WRITE_MASK == WRITE_MASK {
            Op::Write((byte & !WRITE_MASK) as usize)
        } else {
            Op::Read(byte as usize)
        }
    }
}

