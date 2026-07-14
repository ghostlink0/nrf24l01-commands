[![Latest Version](https://img.shields.io/crates/v/nrf24l01-commands.svg)](https://crates.io/crates/nrf24l01-commands)
[![Documentation](https://docs.rs/nrf24l01-commands/badge.svg)](https://docs.rs/nrf24l01-commands)
[![Github Actions](https://github.com/ghostlink0/nrf24l01-commands/workflows/Rust/badge.svg)](https://github.com/ghostlink0/nrf24l01-commands/actions)

# nRF24L01+ Commands

The nRF24L01+ is a wideband 2.4Ghz transceiver IC. It is controlled by commands sent over SPI.

This crate provides:
- Bitfield definitions for nRF24L01+ registers
- A friendly API for generating SPI byte sequences for nRF24L01+ commands

This crate is based on the [nRF24L01+ specification](https://docs.nordicsemi.com/bundle/nRF24L01P_PS_v1.0/resource/nRF24L01P_PS_v1.0.pdf) document.

## Examples

### Generate Bytes to Write CONFIG register
```rust
use nrf24l01_commands::{fields, registers};

const CONFIG: registers::Config = registers::Config::new()
    .with_mask_rx_dr(true)
    .with_mask_tx_ds(false)
    .with_mask_max_rt(false)
    .with_en_crc(false)
    .with_crco(fields::Crco::TwoByte)
    .with_pwr_up(true)
    .with_prim_rx(false);

// Generate SPI byte sequence
const SPI_BYTES: [u8; 2] = CONFIG.as_write_bytes();
assert_eq!(SPI_BYTES, [0b0010_0000, 0b0100_0110]);
```
### Generate Bytes to Read FIFO_STATUS register
```rust
use nrf24l01_commands::registers;

// Generate SPI byte sequence
const SPI_BYTES: [u8; 2] = registers::FifoStatus::as_read_bytes();
assert_eq!(SPI_BYTES, [0x17, 0]);
```
### Write TX payload
```rust
use nrf24l01_commands::*;

const PAYLOAD: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
// Construct SPI byte sequence
const SPI_BYTES: [u8; 10] = [W_TX_PAYLOAD, PAYLOAD[0], PAYLOAD[1], PAYLOAD[2], PAYLOAD[3], PAYLOAD[4], PAYLOAD[5], PAYLOAD[6], PAYLOAD[7], PAYLOAD[8]];
assert_eq!(SPI_BYTES, [0b1010_0000, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
```

### Read RX-payload
```rust
use nrf24l01_commands::*;

assert_eq!(RRxPayload::command_bytes::<6>(), [0b0110_0001, 0, 0, 0, 0, 0]);
```

### Write/read address register
```rust
use nrf24l01_commands::registers::*;

// Write address register
const TX_ADDR_WIDTH4: TxAddr<4> = TxAddr::from_bits(0x170F431EDC);
const WRITE_REG_BYTES: [u8; 5] = TX_ADDR_WIDTH4.as_write_bytes();
assert_eq!(WRITE_REG_BYTES, [0b0010_0000 | 0x10, 0xDC, 0x1E, 0x43, 0x0F]);

// Read address register
assert_eq!(TxAddr::<3>::as_read_bytes(), [0x10, 0, 0, 0]);
```

### Inspect register fields
```rust
use nrf24l01_commands::{fields, registers};

const RF_SETUP: registers::RfSetup = registers::RfSetup::from_bits(0b0010_0010);
// Inspect fields
assert!(!RF_SETUP.cont_wave());
assert!(RF_SETUP.rf_dr_low());
assert!(!RF_SETUP.pll_lock());
assert_eq!(RF_SETUP.rf_dr_high(), fields::RfDrHigh::Mbps1);
assert_eq!(RF_SETUP.rf_pwr(), fields::RfPower::Neg12Dbm);
```
