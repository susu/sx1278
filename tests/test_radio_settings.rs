extern crate sx1278;
mod utils;

mod set_radio_settings {
    use utils::create_sx1278;
    use sx1278::lora::{RadioSettings, Bandwidth, ErrorCoding, SpreadingFactor};

    const SETTINGS: RadioSettings = RadioSettings {
        f_rf: 0x42ee55,
        bandwidth: Bandwidth::_31_25kHz,
        spreading_factor: SpreadingFactor::SF11,
        error_coding: ErrorCoding::_4_7,
        crc: true,
        implicit_header: true,
        payload_length: 42,
        symbol_timeout: 1001,
        low_datarate_optimize: true,
    };

    #[test]
    fn it_should_set_f_rf() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x06), 0x42);
        assert_eq!(device.borrow().registry_value(0x07), 0xee);
        assert_eq!(device.borrow().registry_value(0x08), 0x55);
    }

    #[test]
    fn it_should_set_bandwith() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x1d) & 0b1111_0000, 0b0100_0000);
    }

    #[test]
    fn it_should_set_spreading_factor() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x1e) >> 4, 11);
    }

    #[test]
    fn it_should_set_error_coding_rate() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x1d) & 0b0000_1110, 0b0000_0110);
    }

    #[test]
    fn it_should_set_crc_flag() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x1e) & 0b0000_0100, 0b0000_0100);
    }

    #[test]
    fn it_should_set_implicit_header_flag() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x1d) & 0b0000_0001, 0b0000_0001);
    }

    #[test]
    fn it_should_set_the_payload_length() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x22), 42);
    }

    #[test]
    fn it_should_set_symbol_timeout() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();

        let timeout: u16 =
            ((device.borrow().registry_value(0x1e) & 0b0000_0011) as u16) << 8 |
             device.borrow().registry_value(0x1f) as u16;
        assert_eq!(timeout, SETTINGS.symbol_timeout);
    }

    #[test]
    fn it_should_return_error_for_too_large_symbol_timeout() {
        let (mut sx, _) = create_sx1278();
        let settings = RadioSettings {
            f_rf: 0x42ee55,
            bandwidth: Bandwidth::_31_25kHz,
            spreading_factor: SpreadingFactor::SF11,
            error_coding: ErrorCoding::_4_7,
            crc: true,
            implicit_header: true,
            payload_length: 42,
            symbol_timeout: 1024,
            low_datarate_optimize: true,
        };
        assert!(sx.set_radio_settings(&settings).is_err());
    }

    #[test]
    fn it_should_set_lowdatarate_optimize_flag() {
        let (mut sx, device) = create_sx1278();
        sx.set_radio_settings(&SETTINGS).unwrap();
        assert_eq!(device.borrow().registry_value(0x26) & 0b0000_1000, 0b0000_1000);
    }
}
