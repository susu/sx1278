extern crate sx1278;

#[macro_use]
mod utils;

mod set_pa_settings {
    use utils::create_sx1278;
    use sx1278::lora::PaSettings;

    #[test]
    fn with_pa_boost_and_normal_dac() {
        let (mut sx, device) = create_sx1278();
        let settings = PaSettings::with_pa_boost(17).unwrap();
        sx.set_pa_settings(&settings).unwrap();

        // PaConfig                                   maxpower vvv
        registry_eq!(device.borrow().registry_value(0x09), 0b1_111_1100);
        //                                          PA_BOOST ^     ^^^^ power: 17 - 5 = 12
        // PaDac: 17: stay normalpower
        registry_eq!(device.borrow().registry_value(0x4d) & 0b0000_0111, 0x04);
    }

    #[test]
    fn with_pa_boost_and_highpower_dac() {
        let (mut sx, device) = create_sx1278();
        let settings = PaSettings::with_pa_boost(20).unwrap();
        sx.set_pa_settings(&settings).unwrap();

        registry_eq!(device.borrow().registry_value(0x09), 0b1_111_1111);
        //                                          PA_BOOST ^     ^^^^ power: 20 - 5 = 12
        // PaDac: 20: switch to highpower
        registry_eq!(device.borrow().registry_value(0x4d) & 0b0000_0111, 0x07);
    }

    #[test]
    fn with_rfo() {
        let (mut sx, device) = create_sx1278();
        let settings = PaSettings::with_rfo_output(10).unwrap();
        sx.set_pa_settings(&settings).unwrap();

        registry_eq!(device.borrow().registry_value(0x09), 0b0_111_1011);
        //                                               RFO ^     ^^^^ power: 10 + 1 = 11
        // PaDac: RFO, stay normalpower
        registry_eq!(device.borrow().registry_value(0x4d) & 0b0000_0111, 0x04);
    }
}
