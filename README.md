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

### Command to write CONFIG register
```rust
use nrf24l01_commands::{commands, fields, registers};

const CONFIG: registers::Config = registers::Config::new()
    .with_mask_rx_dr(true)
    .with_mask_tx_ds(false)
    .with_mask_max_rt(false)
    .with_en_crc(false)
    .with_crco(fields::Crco::TwoByte)
    .with_pwr_up(true)
    .with_prim_rx(false);
const WRITE_COMMAND: commands::WRegister<registers::Config> = commands::WRegister(CONFIG);

// Generate SPI byte sequence
const SPI_BYTES: [u8; 2] = WRITE_COMMAND.bytes();
assert_eq!(SPI_BYTES, [0b0010_0000, 0b0100_0110]);
```
### Command to read FIFO_STATUS register
```rust
use nrf24l01_commands::{registers, commands};

// Generate SPI byte sequence
const SPI_BYTES: [u8; 2] = commands::RRegister::<registers::FifoStatus>::bytes();
assert_eq!(SPI_BYTES, [0 | 0x17, 0]);
```
### Command to write TX payload
```rust
use nrf24l01_commands::commands;

const PAYLOAD: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
// Generate SPI byte sequence
const SPI_BYTES: [u8; 10] = commands::WTxPayload(PAYLOAD).bytes();
assert_eq!(SPI_BYTES, [0b1010_0000, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
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
