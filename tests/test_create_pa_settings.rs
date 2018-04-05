extern crate sx1278;

mod create_pa_settings {
    mod with_pa_boost {
        use sx1278::lora::{PaSettings, PaError, PaOutput, PaDac};

        #[test]
        fn power_boundary_tests() {
            assert_eq!(PaSettings::with_pa_boost(-2), Err(PaError::PowerOutOfRange));
            assert_eq!(PaSettings::with_pa_boost(14), Err(PaError::PowerOutOfRange));
            assert!(PaSettings::with_pa_boost(15).is_ok());
            assert!(PaSettings::with_pa_boost(20).is_ok());
            assert_eq!(PaSettings::with_pa_boost(21), Err(PaError::PowerOutOfRange));
        }

        #[test]
        fn should_enable_pa_boost_output() {
            let settings = PaSettings::with_pa_boost(16).unwrap();
            assert_eq!(settings.output, PaOutput::PaBoost);
        }

        #[test]
        fn should_enable_highpower_pa_dac_above_17() {
            let settings = PaSettings::with_pa_boost(18).unwrap();
            assert_eq!(settings.pa_dac, PaDac::HighPower);
        }

        #[test]
        fn should_disable_highpower_pa_dac_below_18() {
            let settings = PaSettings::with_pa_boost(17).unwrap();
            assert_eq!(settings.pa_dac, PaDac::NormalPower);
        }
    }

    mod with_rfo {
        use sx1278::lora::{PaSettings, PaError, PaOutput, PaDac};

        #[test]
        fn power_boundary_tests() {
            assert_eq!(PaSettings::with_rfo_output(-2), Err(PaError::PowerOutOfRange));
            assert!(PaSettings::with_rfo_output(-1).is_ok());
            assert!(PaSettings::with_rfo_output(14).is_ok());
            assert_eq!(PaSettings::with_rfo_output(15), Err(PaError::PowerOutOfRange));
        }

        #[test]
        fn should_enable_rfo_output_and_disable_highpower_padac() {
            let settings = PaSettings::with_rfo_output(-1).unwrap();
            assert_eq!(settings.output, PaOutput::Rfo);
            assert_eq!(settings.pa_dac, PaDac::NormalPower);
        }
    }
}
