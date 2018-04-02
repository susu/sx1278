extern crate sx1278;
mod utils;

use utils::create_sx1278;

mod change_mode {
    use super::*;
    use sx1278::lora::Mode;

    #[test]
    fn to_sleep() {
        let (mut sx, device) = create_sx1278();
        sx.set_mode(Mode::Sleep).unwrap();

        assert_eq!(sx.mode().unwrap(), Mode::Sleep);
        assert_eq!(device.borrow().registry_value(0x01) & 0b0000_0111, 0b0000_0000);
    }

    #[test]
    fn to_rxcont() {
        let (mut sx, device) = create_sx1278();
        sx.set_mode(Mode::RxCont).unwrap();

        assert_eq!(sx.mode().unwrap(), Mode::RxCont);
        assert_eq!(device.borrow().registry_value(0x01) & 0b0000_0111, 0b0000_0101);
    }

    #[test]
    fn to_tx() {
        let (mut sx, device) = create_sx1278();
        sx.set_mode(Mode::Tx).unwrap();

        assert_eq!(sx.mode().unwrap(), Mode::Tx);
        assert_eq!(device.borrow().registry_value(0x01) & 0b0000_0111, 0b0000_0011);
    }
}
