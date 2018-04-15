extern crate sx1278;

#[macro_use]
mod utils;

mod tx {
    use utils::create_sx1278;

    #[test]
    fn test_transmit_packet() {
        let (mut sx, device) = create_sx1278();

        let payload = [42u8, 99, 255];

        sx.transmit_packet(&payload).unwrap();

        let dev = device.borrow();

        // FIFO ptr set to TX_BASE_ADDR
        let tx_base_addr = dev.registry_value(0x0e);
        registry_eq!(dev.registry_value(0x0d), tx_base_addr);

        // assert: data is written to FIFO
        let start = tx_base_addr as usize;
        let end = tx_base_addr as usize + payload.len();
        assert_eq!(dev.fifo[start..end], payload[..]);

        // assert: payload length set
        assert_eq!(dev.registry_value(0x22) as usize, payload.len());

        // assert: mode switched to TX
        assert_eq!(dev.registry_value(0x01) & 0b0000_0111, 0b0000_0011);
    }
}
