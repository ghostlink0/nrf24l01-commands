#![no_std]
#![doc = include_str!("../README.md")]

pub mod fields;
pub mod registers;

/// Command word for `W_REGISTER`. Bitwise-OR with register address to write the register.
pub const W_REGISTER: u8 = 0b0010_0000;

/// Read RX payload.
///
/// #### Const Parameter `N`
/// `N - 1` is the number of payload bytes to read.
///
/// <div class="warning">
/// Must be 2 to 33 bytes.
/// </div>
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// assert_eq!(RRxPayload::command_bytes::<4>(), [0b0110_0001, 0, 0, 0]);
/// ```
pub struct RRxPayload;

impl RRxPayload {
    const WORD: u8 = 0b0110_0001;

    pub const fn command_bytes<const N: usize>() -> [u8; N] {
        const { assert!(2 <= N && N <= 33) };
        let mut bytes = [0u8; N];
        bytes[0] = Self::WORD;
        bytes
    }
}

/// Write TX payload.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// // Manually construct buffer
/// const BUFFER: [u8; 8] = [W_TX_PAYLOAD, 1, 2, 3, 4, 5, 6, 7];
/// ```
pub const W_TX_PAYLOAD: u8 = 0b1010_0000;

/// Flush TX FIFO. Used in TX mode.
pub const FLUSH_TX: u8 = 0b1110_0001;

/// Flush RX FIFO. Used in RX mode.
pub const FLUSH_RX: u8 = 0b1110_0010;

/// Reuse last transmitted payload. Packets are repeatedly transmitted as long
/// as CE is high. TX payload reuse is active until [`W_TX_PAYLOAD`] or [`FLUSH_TX`]
/// is executed.
pub const REUSE_TX_PL: u8 = 0b1110_0011;

/// Read RX payload width for the top payload in RX FIFO.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// assert_eq!(R_RX_PL_WID, [0b0110_0000, 0]);
/// ```
pub const R_RX_PL_WID: [u8; 2] = [0b0110_0000, 0];

/// Write payload to be transmitted with ACK packet on a data pipe. Used in RX mode.
/// Maximum three ACK packet payloads can be pending. Payloads with the same pipe
/// are handled first-in-first-out.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// const PIPE: u8 = 4;
/// // Manually construct buffer
/// const BUFFER: [u8; 5] = [w_ack_payload_word(PIPE), 1, 2, 3, 4];
/// assert_eq!(BUFFER, [0b1010_1100, 1, 2, 3, 4]);
/// ```
pub const W_ACK_PAYLOAD: u8 = 0b1010_1000;

/// Write payload to be transmitted with ACK packet on a data pipe. Used in RX mode.
/// Maximum three ACK packet payloads can be pending. Payloads with the same pipe
/// are handled first-in-first-out.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// const PIPE: u8 = 4;
/// // Manually construct buffer
/// const BUFFER: [u8; 5] = [w_ack_payload_word(PIPE), 1, 2, 3, 4];
/// assert_eq!(BUFFER, [0b1010_1100, 1, 2, 3, 4]);
/// ```
pub const fn w_ack_payload_word(pipe: u8) -> u8 {
    W_ACK_PAYLOAD | pipe
}

/// Write TX payload with AUTOACK disabled.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// // Manually construct buffer
/// const BUFFER: [u8; 8] = [W_TX_PAYLOAD_NOACK, 1, 2, 3, 4, 5, 6, 7];
/// ```
pub const W_TX_PAYLOAD_NOACK: u8 = 0b1011_0000;

/// No operation. Used to read the status register.
pub const NOP: u8 = 0b1111_1111;
