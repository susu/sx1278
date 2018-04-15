
extern crate sx1278;

#[macro_use]
mod utils;

mod irq {
    use utils::create_sx1278;
    use sx1278::lora::Irq;

    #[test]
    fn return_single_flag() {
        let (mut sx, device) = create_sx1278();

        device.borrow_mut().registry[0x12] = 0b0000_1000;

        let flags = sx.irq_flags().unwrap();
        assert_eq!(flags, Irq::TX_DONE);
    }
}
