extern crate sx1278;

#[macro_use]
mod utils;

mod rx {
    use utils::create_sx1278;
    use sx1278::lora::Mode;

    #[test]
    fn test_begin_receive_single_should_set_fifo_and_mode() {
        let (mut sx, dev) = create_sx1278();
        sx.begin_receive_single().unwrap();

        // Fifo ptr set to RX_BASE_ADDR
        let rx_base_addr = dev.borrow().registry_value(0x0f);
        registry_eq!(dev.borrow().registry_value(0x0d), rx_base_addr);

        // RX SINGLE mode
        registry_eq!(dev.borrow().registry_value(0x01) & 0b0000_0111, 0b0000_0110);
        assert_eq!(sx.mode().unwrap(), Mode::RxSingle);
    }

    #[test]
    fn test_read_packet_should_return_the_packet() {
        let (mut sx, device) = create_sx1278();

        // prepare mock:
        // 1. set data in FIFO
        // 2. set received payload length
        // 3. set RX IRQ
        let rx_base_addr = device.borrow().registry_value(0x0f);
        device.borrow_mut().set_fifo_value(rx_base_addr, 0x42);
        device.borrow_mut().set_fifo_value(rx_base_addr + 1, 0x43);
        device.borrow_mut().set_fifo_value(rx_base_addr + 2, 0x44);
        device.borrow_mut().set_fifo_value(rx_base_addr + 3, 0x45);

        // sx.read_packet()
    }

    // TODO invalid header check? CRC check?
}
