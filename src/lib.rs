#![no_std]
#![doc = include_str!("../README.md")]

pub mod fields;
pub mod registers;

/// Write command and status registers.
pub struct WRegister;
impl WRegister {
    pub const WORD: u8 = 0b0010_0000;
}

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
/// assert_eq!(RRxPayload::as_bytes::<4>(), [0b0110_0001, 0, 0, 0]);
/// ```
pub struct RRxPayload;
impl RRxPayload {
    pub const WORD: u8 = 0b0110_0001;

    pub const fn as_bytes<const N: usize>() -> [u8; N] {
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
/// const BUFFER: [u8; 8] = [WTxPayload::WORD, 1, 2, 3, 4, 5, 6, 7];
/// ```
pub struct WTxPayload;
impl WTxPayload {
    pub const WORD: u8 = 0b1010_0000;
}

/// Flush TX FIFO. Used in TX mode.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// assert_eq!(FlushTx::as_bytes(), [0b1110_0001]);
/// ```
pub struct FlushTx;
impl FlushTx {
    pub const WORD: u8 = 0b1110_0001;

    pub const fn as_bytes() -> [u8; 1] {
        [Self::WORD]
    }
}

/// Flush RX FIFO. Used in RX mode.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// assert_eq!(FlushRx::as_bytes(), [0b1110_0010]);
/// ```
pub struct FlushRx;
impl FlushRx {
    pub const WORD: u8 = 0b1110_0010;

    pub const fn as_bytes() -> [u8; 1] {
        [Self::WORD]
    }
}

/// Reuse last transmitted payload. Packets are repeatedly transmitted as long
/// as CE is high. TX payload reuse is active until [`WTxPayload`] or [`FlushTx`]
/// is executed.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// assert_eq!(ReuseTxPl::as_bytes(), [0b1110_0011]);
/// ```
pub struct ReuseTxPl;
impl ReuseTxPl {
    pub const WORD: u8 = 0b1110_0011;

    pub const fn as_bytes() -> [u8; 1] {
        [Self::WORD]
    }
}

/// Read RX payload width for the top payload in RX FIFO.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// assert_eq!(RRxPlWid::as_bytes(), [0b0110_0000, 0]);
/// ```
pub struct RRxPlWid;
impl RRxPlWid {
    pub const WORD: u8 = 0b0110_0000;

    pub const fn as_bytes() -> [u8; 2] {
        [Self::WORD, 0]
    }
}

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
/// const BUFFER: [u8; 5] = [WAckPayload::word(PIPE), 1, 2, 3, 4];
/// assert_eq!(BUFFER, [0b1010_1100, 1, 2, 3, 4]);
/// ```
pub struct WAckPayload;
impl WAckPayload {
    pub const WORD: u8 = 0b1010_1000;

    pub const fn word(pipe: u8) -> u8 {
        Self::WORD | pipe
    }
}

/// Write TX payload with AUTOACK disabled.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// // Manually construct buffer
/// const BUFFER: [u8; 8] = [WTxPayloadNoAck::WORD, 1, 2, 3, 4, 5, 6, 7];
/// ```
pub struct WTxPayloadNoAck;
impl WTxPayloadNoAck {
    pub const WORD: u8 = 0b1011_0000;
}

/// No operation. Used to read the status register.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::*;
///
/// assert_eq!(Nop::WORD, 0b1111_1111);
/// ```
pub struct Nop;
impl Nop {
    pub const WORD: u8 = 0b1111_1111;
}
