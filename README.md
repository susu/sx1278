# SX1278

**WORK-IN-PROGRESS**

Platform agnostic driver for [Semtech]'s [SX1278] (LoRa RF chip).

Implemented over [`embedded-hal`] traits.

Recommended read: [SX1278 datasheet]

[SX1278]: https://www.semtech.com/products/wireless-rf/lora-transceivers/SX1278
[Semtech]: https://www.semtech.com/
[`embedded-hal`]: https://crates.io/crates/embedded-hal
[SX1278 datasheet]: https://www.semtech.com/uploads/documents/DS_SX1276-7-8-9_W_APP_V5.pdf


I plan to implement only the LoRa functionality, not the FSK/OOK.

## TODO list - LoRa mode only

- [x] Set mode
- [x] Set up radio frontend (frequency, BW, SF, EC, ...)
- [x] Set up PA (Power Amp for output)
- [ ] Set up receiver config (LNA/AGC)
- [ ] "Easy API" for transmitting data
- [ ] "Easy API" for receiving data
- [ ] Query modem status
- [ ] DIOx ping configuration and handle possible interrupts
