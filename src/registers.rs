//! Register bitfields for the nRF24L01+.
//!
//! ## Example with the CONFIG register
//! ```rust
//! use nrf24l01_commands::{fields::*, registers::*};
//!
//! // Default value
//! let reg = Config::new();
//! assert_eq!(reg.into_bits(), 0b0000_1000);
//!
//! // Read fields
//! let reg = Config::from_bits(0b0000_0110);
//! assert!(!reg.mask_rx_dr());
//! assert!(!reg.mask_tx_ds());
//! assert!(!reg.mask_max_rt());
//! assert!(!reg.en_crc());
//! assert_eq!(reg.crco(), Crco::TwoByte);
//! assert!(reg.pwr_up());
//! assert!(!reg.prim_rx());
//!
//! // Write fields
//! let reg = Config::new()
//!     .with_mask_rx_dr(true)
//!     .with_mask_tx_ds(false)
//!     .with_mask_max_rt(false)
//!     .with_en_crc(false)
//!     .with_crco(Crco::TwoByte)
//!     .with_pwr_up(true)
//!     .with_prim_rx(false);
//! assert_eq!(reg.into_bits(), 0b0100_0110);
//! ```
//!
//! ## Example with TX_ADDR register
//! ```rust
//! use nrf24l01_commands::registers::*;
//!
//! // For multi-byte registers (RxAddrP0, RxAddrP1, TxAddr)
//! // Generate register read bytes
//! assert_eq!(TxAddr::<4>::as_read_bytes(), [0x10, 0, 0, 0, 0]);
//! // Generate register write bytes
//! const TX_ADDR_WIDTH5: TxAddr<5> = TxAddr::from_bits(0x170F431EDC);
//! const WRITE_REG_BYTES: [u8; 6] = TX_ADDR_WIDTH5.as_write_bytes();
//! assert_eq!(WRITE_REG_BYTES, [0b0010_0000 | 0x10, 0xDC, 0x1E, 0x43, 0x0F, 0x17]);
//! ```
use crate::fields::*;
use bitfield_struct::bitfield;

/// Implement methods for a register bitfield struct.
macro_rules! impl_register {
    ($type:ty, $addr:literal) => {
        impl $type {
            pub const ADDRESS: u8 = $addr;

            /// Get the register's address.
            pub const fn address(&self) -> u8 {
                Self::ADDRESS
            }
            /// Get the SPI byte sequence for a write command for this register.
            pub const fn as_write_bytes(&self) -> [u8; 2] {
                [$crate::WRegister::WORD | Self::ADDRESS, self.into_bits()]
            }
            /// Get the SPI byte sequence for a read command for this register.
            pub const fn as_read_bytes() -> [u8; 2] {
                [Self::ADDRESS, 0]
            }
        }
    };
}

/// Implement methods for an address register (contains 3-5 data bytes)
macro_rules! impl_address_register {
    ($type:ident, $inner:ident, $addr:literal, $getter:ident, $setter:ident, $doc:literal, [$($n:literal),+]) => {
        impl<const N: usize> $type<N> {
            pub const ADDRESS: u8 = $addr;

            pub const fn new() -> Self {
                Self($inner::new())
            }

            pub const fn from_bits(bits: u64) -> Self {
                Self($inner::from_bits(bits))
            }

            const fn into_bytes(self) -> [u8; N] {
                address_into_bytes(self.0.0)
            }

            #[doc = $doc]
            pub const fn $getter(&self) -> u64 {
                self.0.$getter()
            }

            #[doc = $doc]
            pub const fn $setter(mut self, value: u64) -> Self {
                self.0 = self.0.$setter(value);
                self
            }
        }

        impl<const N: usize> Default for $type<N> {
            fn default() -> Self {
                Self::new()
            }
        }

        /// Implement `as_write_bytes` and `as_read_bytes` for each address width.
        $(
            impl $type<$n> {
                /// Get the SPI byte sequence for a write command for this register.
                pub const fn as_write_bytes(&self) -> [u8; $n + 1] {
                    let addr = self.into_bytes();
                    let mut bytes = [0u8; $n + 1];
                    bytes[0] = $crate::WRegister::WORD | Self::ADDRESS;
                    let mut i = 0;
                    while i < $n {
                        bytes[i + 1] = addr[i];
                        i += 1;
                    }
                    bytes
                }
                /// Get the SPI byte sequence for a read command for this register.
                pub const fn as_read_bytes() -> [u8; $n + 1] {
                    let mut bytes = [0u8; $n + 1];
                    bytes[0] = Self::ADDRESS;
                    bytes
                }
            }
        )*
    };
}

/// # CONFIG register
///
/// Address = `0x00`
///
/// ## Fields
///
/// #### `mask_rx_dr` | bit 6
/// Mask/unmask interrupt caused by __RX_DR__.
///
/// `0`: unmasked, interrupt reflected on IRQ
///
/// `1`: masked, interrupt not reflected on IRQ
///
/// #### `mask_tx_ds` | bit 5
/// Mask/unmask interrupt caused by __TX_DS__.
///
/// `0`: unmasked, interrupt reflected on IRQ
///
/// `1`: masked, interrupt not reflected on IRQ
///
/// #### `mask_max_rt` | bit 4
/// Mask/unmask interrupt caused by __MAX_RT__.
///
/// `0`: unmasked, interrupt reflected on IRQ
///
/// `1`: masked, interrupt not reflected on IRQ
///
/// #### `en_crc` | bit 3
/// Enable/disable CRC. Default value: `1` (enabled)
///
/// #### `crco` | bit 2
/// CRC encoding scheme. Enum: [`Crco`].
///
/// #### `pwr_up` | bit 1
/// Power down/up.
///
/// `0`: Power down
///
/// `1`: Power up
///
/// #### `prim_rx` | bit 0
/// Set primary TX/RX.
///
/// `0`: primary TX
///
/// `1`: primary RX
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::{fields::*, registers::*};
///
/// // Default value
/// let reg = Config::new();
/// assert_eq!(reg.into_bits(), 0b0000_1000);
///
/// // Write fields
/// let reg = Config::new()
///     .with_mask_rx_dr(true)
///     .with_mask_tx_ds(false)
///     .with_mask_max_rt(false)
///     .with_en_crc(false)
///     .with_crco(Crco::TwoByte)
///     .with_pwr_up(true)
///     .with_prim_rx(false);
/// assert_eq!(reg.into_bits(), 0b0100_0110);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Config {
    #[bits(1)]
    __: bool,

    /// Mask/unmask interrupt caused by __RX_DR__.
    ///
    /// `0`: unmasked, interrupt reflected on IRQ
    ///
    /// `1`: masked, interrupt not reflected on IRQ
    #[bits(1)]
    pub mask_rx_dr: bool,

    /// Mask/unmask interrupt caused by __TX_DS__.
    ///
    /// `0`: unmasked, interrupt reflected on IRQ
    ///
    /// `1`: masked, interrupt not reflected on IRQ
    #[bits(1)]
    pub mask_tx_ds: bool,

    /// Mask/unmask interrupt caused by __MAX_RT__.
    ///
    /// `0`: unmasked, interrupt reflected on IRQ
    ///
    /// `1`: masked, interrupt not reflected on IRQ
    #[bits(1)]
    pub mask_max_rt: bool,

    /// Enable/disable CRC. Default value: `1` (enabled)
    #[bits(1, default = true)]
    pub en_crc: bool,

    /// CRC encoding scheme. Enum: [`Crco`].
    #[bits(1)]
    pub crco: Crco,

    /// Power down/up.
    ///
    /// `0`: Power down
    ///
    /// `1`: Power up
    #[bits(1)]
    pub pwr_up: bool,

    /// Set primary TX/RX.
    ///
    /// `0`: primary TX
    ///
    /// `1`: primary RX
    #[bits(1)]
    pub prim_rx: bool,
}
impl_register!(Config, 0x00);

/// # EN_AA register
/// Enable 'Auto Acknowledgement' on data pipes 0-5.
///
/// Address = `0x01`
///
/// ## Fields
/// All fields default to 1.
///
/// #### `enaa_pN` | bit `N`
/// Enable 'Auto Acknowledgement' on data pipes `N` = 0-5.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = EnAa::new();
/// assert_eq!(reg.into_bits(), 0b0011_1111);
///
/// // Write fields
/// let reg = EnAa::new()
///     .with_enaa_p5(true)
///     .with_enaa_p4(true)
///     .with_enaa_p3(false)
///     .with_enaa_p2(false)
///     .with_enaa_p1(false)
///     .with_enaa_p0(false);
/// assert_eq!(reg.into_bits(), 0b0011_0000);
/// ```
#[bitfield(u8, order = Msb)]
pub struct EnAa {
    #[bits(2)]
    __: u8,
    /// Enable 'Auto Acknowledgement' on data pipe 5.
    #[bits(1, default = true)]
    pub enaa_p5: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 4.
    #[bits(1, default = true)]
    pub enaa_p4: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 3.
    #[bits(1, default = true)]
    pub enaa_p3: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 2.
    #[bits(1, default = true)]
    pub enaa_p2: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 1.
    #[bits(1, default = true)]
    pub enaa_p1: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 0.
    #[bits(1, default = true)]
    pub enaa_p0: bool,
}
impl_register!(EnAa, 0x01);

/// # EN_RXADDR register
/// Enable RX address on data pipes 0-5.
///
/// Address = `0x02`
///
/// ## Fields
///
/// `erx_p0` and `erx_p1` default to 1.
///
/// #### `erx_pN` | bit `N`
/// Enable RX adddress on data pipes `N` = 0-5.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = EnRxaddr::new();
/// assert_eq!(reg.into_bits(), 0b0000_0011);
///
/// // Write fields
/// let reg = EnRxaddr::new()
///     .with_erx_p5(true)
///     .with_erx_p4(false)
///     .with_erx_p3(false)
///     .with_erx_p2(false)
///     .with_erx_p1(true)
///     .with_erx_p0(false);
/// assert_eq!(reg.into_bits(), 0b0010_0010);
/// ```
#[bitfield(u8, order = Msb)]
pub struct EnRxaddr {
    #[bits(2)]
    __: u8,
    /// Enable RX address for data pipe 5.
    #[bits(1)]
    pub erx_p5: bool,
    /// Enable RX address for data pipe 4.
    #[bits(1)]
    pub erx_p4: bool,
    /// Enable RX address for data pipe 3.
    #[bits(1)]
    pub erx_p3: bool,
    /// Enable RX address for data pipe 2.
    #[bits(1)]
    pub erx_p2: bool,
    /// Enable RX address for data pipe 1.
    #[bits(1, default = true)]
    pub erx_p1: bool,
    /// Enable RX address for data pipe 0.
    #[bits(1, default = true)]
    pub erx_p0: bool,
}
impl_register!(EnRxaddr, 0x02);

/// # SETUP_AW register
/// Set up address width. This applies to [`TxAddr`] and all RX addresses for data pipes.
///
/// Address = `0x03`
///
/// ## Fields
///
/// #### `aw` | bits 1:0
/// Address width. Default value: `11` (5 byte address).
/// Enum: [`AddressWidth`].
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::{fields::*, registers::*};
///
/// // Default value
/// let reg = SetupAw::new();
/// assert_eq!(reg.into_bits(), 0b0000_0011);
///
/// // Write fields
/// let reg = SetupAw::new().with_aw(AddressWidth::FourByte);
/// assert_eq!(reg.into_bits(), 0b0000_0010);
/// ```
#[bitfield(u8, order = Msb)]
pub struct SetupAw {
    #[bits(6)]
    __: u8,

    /// Address width. Default value: `11` (5 byte address).
    /// Enum: [`AddressWidth`].
    #[bits(2, default = AddressWidth::FiveByte)]
    pub aw: AddressWidth,
}
impl_register!(SetupAw, 0x03);

/// # SETUP_RETR register
/// Set up 'Automatic Retransmission'.
///
/// Address = `0x04`
///
/// ## Fields
///
/// #### `ard` | bits 7:4
/// Auto retransmit delay. Enum: [`AutoRetransmitDelay`].
///
/// #### `arc` | bits 3:0
/// Maximum auto retransmits. Default value: `0011` (3 retransmits)
///
/// `0000`: Auto retransmit disabled
///
/// `0001`: Up to 1 retransmit
///
/// `0010`: Up to 2 retransmits
///
/// ……
///
/// `1111`: Up to 15 retransmits
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::{fields::*, registers::*};
///
/// // Default value
/// let reg = SetupRetr::new();
/// assert_eq!(reg.into_bits(), 0b0000_0011);
///
/// // Write fields
/// let reg = SetupRetr::new()
///     .with_ard(AutoRetransmitDelay::US750)
///     .with_arc(0b1111);
/// assert_eq!(reg.into_bits(), 0b0010_1111);
/// ```
#[bitfield(u8, order = Msb)]
pub struct SetupRetr {
    /// Auto retransmit delay. Enum: [`AutoRetransmitDelay`].
    #[bits(4)]
    pub ard: AutoRetransmitDelay,

    /// Maximum auto retransmits. Default value: `0011` (3 retransmits)
    ///
    /// `0000`: Auto retransmit disabled
    ///
    /// `0001`: Up to 1 retransmit
    ///
    /// `0010`: Up to 2 retransmits
    ///
    /// ……
    ///
    /// `1111`: Up to 15 retransmits
    #[bits(4, default = 3)]
    pub arc: u8,
}
impl_register!(SetupRetr, 0x04);

/// # RF_CH register
/// Set RF channel.
///
/// Address = `0x05`
///
/// ## Fields
/// #### `rf_ch` | bits 6:0
/// Sets the frequency channel to operate on 0 - 125. Default value: `2`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RfCh::new();
/// assert_eq!(reg.into_bits(), 0b0000_0010);
///
/// // Write fields
/// let reg = RfCh::new().with_rf_ch(89);
/// assert_eq!(reg.into_bits(), 89);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RfCh {
    #[bits(1)]
    __: bool,

    /// Sets the frequency channel to operate on 0 - 125. Default value: `2`.
    #[bits(7, default = 2)]
    pub rf_ch: u8,
}
impl_register!(RfCh, 0x05);

/// # RF_SETUP register
/// Set RF air data rate and output power.
///
/// Address = `0x06`
///
/// ## Fields
/// #### `cont_wave` | bit 7
/// Enables continuous carrier transmit.
///
/// #### `rf_dr_low` | bit 5
/// Set RF data rate to 250kbps. See `rf_dr_high`.
///
/// #### `pll_lock` | bit 4
/// Force PLL lock signal. Only used in test.
///
/// #### `rf_dr_high` | bit 3
/// Select between the high speed data rates. This bit
/// is don't care if `rf_dr_low` is set. Enum: [`RfDrHigh`].
/// Default value: `1` (2 Mbps).
///
/// Encoding [RF_DR_LOW, RF_DR_HIGH]:
///
/// `00` - 1Mbps
///
/// `01` - 2Mbps
///
/// `10` - 250kbps
///
/// `11` - Reserved
///
/// #### `rf_pwr` | bits 2:1
/// RF output power in TX mode. Enum: [`RfPower`].
/// Default value: `11` (0 dBm).
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::{fields::*, registers::*};
///
/// // Default value
/// let reg = RfSetup::new();
/// assert_eq!(reg.into_bits(), 0b0000_1110);
///
/// // Write fields
/// let reg = RfSetup::new()
///     .with_pll_lock(false)
///     .with_rf_dr_low(true)
///     .with_rf_dr_high(RfDrHigh::Mbps1)
///     .with_rf_pwr(RfPower::Neg6Dbm);
/// assert_eq!(reg.into_bits(), 0b0010_0100);
///
/// // Read fields
/// assert_eq!(reg.rf_pwr(), RfPower::Neg6Dbm);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RfSetup {
    /// Enables continuous carrier transmit.
    #[bits(1)]
    pub cont_wave: bool,

    #[bits(1)]
    __: bool,

    /// Set RF data rate to 250kbps. See `rf_dr_high`.
    #[bits(1)]
    pub rf_dr_low: bool,

    /// Force PLL lock signal. Only used in test.
    #[bits(1)]
    pub pll_lock: bool,

    /// Select between the high speed data rates. This bit
    /// is don't care if `rf_dr_low` is set. Enum: [`RfDrHigh`].
    /// Default value: `1` (2 Mbps).
    ///
    /// Encoding [RF_DR_LOW, RF_DR_HIGH]:
    ///
    /// `00` - 1Mbps
    ///
    /// `01` - 2Mbps
    ///
    /// `10` - 250kbps
    ///
    /// `11` - Reserved
    #[bits(1, default = RfDrHigh::Mbps2)]
    pub rf_dr_high: RfDrHigh,

    /// RF output power in TX mode. Enum: [`RfPower`].
    /// Default value: `11` (0 dBm).
    #[bits(2, default = RfPower::Dbm0)]
    pub rf_pwr: RfPower,

    #[bits(1)]
    __: bool,
}
impl_register!(RfSetup, 0x06);

/// # STATUS register
///
/// Address = `0x07`
///
/// ## Fields
/// #### `rx_dr` | bit 6
/// Data ready RX FIFO interrupt. Asserted when new data arrives in RX FIFO. Write 1 to clear bit.
///
/// #### `tx_ds` | bit 5
/// Data sent TX FIFO interrupt. Asserted when packet is transmitted. If AUTO_ACK is activated, ACK must be received before interrupt goes high. Write 1 to clear bit.
///
/// #### `max_rt` | bit 4
/// Maximum number of TX retransmits interrupt. If MAX_RT is asserted it must be cleared before communication can continue. Write 1 to clear bit.
///
/// #### `rx_p_no` | bits 3:1
/// Data pipe number for the payload available from reading RX FIFO. This field is read-only.
/// Enum: [`RxPipeNo`].
///
/// #### `tx_full` | bit 0
/// TX FIFO full flag. This field is read-only.
///
/// `0`: Not full
///
/// `1`: TX FIFO full
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::{fields::*, registers::*};
///
/// // Default value
/// let reg = Status::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Read fields
/// let reg = Status::from_bits(0b0011_0101);
/// assert!(!reg.rx_dr());
/// assert!(reg.tx_ds());
/// assert!(reg.max_rt());
/// assert_eq!(reg.rx_p_no(), RxPipeNo::Pipe2);
/// assert!(reg.tx_full());
///
/// // Write fields
/// let reg = Status::new()
///     .with_rx_dr(false)
///     .with_tx_ds(true)
///     .with_max_rt(false);
/// assert_eq!(reg.into_bits(), 0b0010_0000);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Status {
    #[bits(1)]
    __: bool,

    /// Data ready RX FIFO interrupt. Asserted when new data arrives in RX FIFO. Write 1 to clear bit.
    #[bits(1)]
    pub rx_dr: bool,

    /// Data sent TX FIFO interrupt. Asserted when packet is transmitted. If AUTO_ACK is activated, ACK must be received before interrupt goes high. Write 1 to clear bit.
    #[bits(1)]
    pub tx_ds: bool,

    /// Maximum number of TX retransmits interrupt. If MAX_RT is asserted it must be cleared before communication can continue. Write 1 to clear bit.
    #[bits(1)]
    pub max_rt: bool,

    /// Data pipe number for the payload available from reading RX FIFO. This field is read-only.
    /// Enum: [`RxPipeNo`].
    #[bits(3, access = RO)]
    pub rx_p_no: RxPipeNo,

    /// TX FIFO full flag. This field is read-only.
    ///
    /// `0`: Not full
    ///
    /// `1`: TX FIFO full
    #[bits(1, access = RO)]
    pub tx_full: bool,
}
impl_register!(Status, 0x07);

/// # OBSERVE_TX register
/// Transmit observe register.
///
/// Address = `0x08`
///
/// #### `plos_cnt` | bits 7:4
/// Count lost packets. This counter is overflow protected to 15,
/// and continues at max until reset. This counter is reset by writing
/// to [`RfCh`]. This field is read-only.
///
/// #### `arc_cnt` | 3:0
/// Count retransmitted packets. The counter is reset when transmission
/// of a new packet starts. This field is read-only.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = ObserveTx::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Read fields
/// let reg = ObserveTx::from_bits(0b1010_1100);
/// assert_eq!(reg.plos_cnt(), 10);
/// assert_eq!(reg.arc_cnt(), 12);
/// ```
#[bitfield(u8, order = Msb)]
pub struct ObserveTx {
    /// Count lost packets. This counter is overflow protected to 15,
    /// and continues at max until reset. This counter is reset by writing
    /// to __RF_CH__. This field is read-only.
    #[bits(4, access = RO)]
    pub plos_cnt: u8,

    /// Count retransmitted packets. The counter is reset when transmission
    /// of a new packet starts. This field is read-only.
    #[bits(4, access = RO)]
    pub arc_cnt: u8,
}
impl_register!(ObserveTx, 0x08);

/// # RPD register
/// Received power detector.
///
/// Address = `0x09`
///
/// ## Fields
/// #### `rpd` | bit 0
/// Triggers at received power levels above -64 dBm that are present
/// in the RF channel you receive on. If the received power is less
/// than -64 dBm, RDP = 0.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// let reg = Rpd::from_bits(1);
/// assert!(reg.rpd());
/// ```
#[bitfield(u8, order = Msb)]
pub struct Rpd {
    #[bits(7)]
    __: u8,

    /// Triggers at received power levels above -64 dBm that are present
    /// in the RF channel you receive on. If the received power is less
    /// than -64 dBm, RDP = 0.
    #[bits(1, access = RO)]
    pub rpd: bool,
}
impl_register!(Rpd, 0x09);

/// # RX_ADDR_P0 register
/// RX address data pipe 0.
///
/// Address = `0x0A`
///
/// Const parameter `N`: address width in bytes.
/// <div class="warning">
/// N must be of {3, 4, 5}.
/// </div>
///
/// ## Fields
/// #### `rx_addr_p0` | bits 39:0
/// RX address data pipe 0. Default value: `0xE7E7E7E7E7`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxAddrP0::<4>::new();
/// assert_eq!(reg.rx_addr_p0(), 0xE7E7E7E7E7);
///
/// // Write fields
/// let reg = RxAddrP0::<5>::new().with_rx_addr_p0(0xC2840DF659);
/// assert_eq!(reg.rx_addr_p0(), 0xC2840DF659);
///
/// // Generate write register bytes
/// assert_eq!(reg.as_write_bytes(), [0x0A | 0b0010_0000, 0x59, 0xF6, 0x0D, 0x84, 0xC2]);
///
/// // For 3 byte address width
/// let reg = RxAddrP0::<3>::new().with_rx_addr_p0(0xC2840DF659);
/// assert_eq!(reg.as_write_bytes(), [0x0A | 0b0010_0000, 0x59, 0xF6, 0x0D]);
/// ```
#[derive(Copy, Clone)]
pub struct RxAddrP0<const N: usize>(RxAddrP0Fields);

#[bitfield(u64, order = Msb)]
struct RxAddrP0Fields {
    #[bits(24)]
    __: u32,

    /// RX address data pipe 0. Default value: `0xE7E7E7E7E7`.
    #[bits(40, default = 0xE7E7E7E7E7)]
    rx_addr_p0: u64,
}

/// Convert u64 address to little-endian bytes.
/// Const parameter `N`: address width in bytes. Constraint: `N` in {3, 4, 5}.
#[inline(always)]
const fn address_into_bytes<const N: usize>(addr: u64) -> [u8; N] {
    let le_bytes: [u8; 8] = addr.to_le_bytes();
    let mut bytes = [0; N];
    let mut i = 0;
    while i < N {
        bytes[i] = le_bytes[i];
        i += 1;
    }
    bytes
}

impl_address_register!(
    RxAddrP0,
    RxAddrP0Fields,
    0x0A,
    rx_addr_p0,
    with_rx_addr_p0,
    "RX address data pipe 0. Default value: `0xE7E7E7E7E7`.",
    [3, 4, 5]
);

/// # RX_ADDR_P1 register
/// RX address data pipe 1.
///
/// Address = `0x0B`
///
/// Const parameter `N`: address width in bytes.
/// <div class="warning">
/// N must be of {3, 4, 5}.
/// </div>
///
/// ## Fields
/// #### `rx_addr_p1` | bits 39:0
/// RX address data pipe 1. Default value: `0xC2C2C2C2C2`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxAddrP1::<4>::new();
/// assert_eq!(reg.rx_addr_p1(), 0xC2C2C2C2C2);
///
/// // Write fields
/// let reg = RxAddrP1::<5>::new().with_rx_addr_p1(0xC2840DF659);
/// assert_eq!(reg.rx_addr_p1(), 0xC2840DF659);
///
/// // Generate write register bytes
/// assert_eq!(reg.as_write_bytes(), [0x0B | 0b0010_0000, 0x59, 0xF6, 0x0D, 0x84, 0xC2]);
///
/// // For 3 byte address width
/// let reg = RxAddrP1::<3>::new().with_rx_addr_p1(0xC2840DF659);
/// assert_eq!(reg.as_write_bytes(), [0x0B | 0b0010_0000, 0x59, 0xF6, 0x0D]);
/// ```
#[derive(Copy, Clone)]
pub struct RxAddrP1<const N: usize>(RxAddrP1Fields);

#[bitfield(u64, order = Msb)]
struct RxAddrP1Fields {
    #[bits(24)]
    __: u32,

    /// RX address data pipe 1. Default value: `0xC2C2C2C2C2`.
    #[bits(40, default = 0xC2C2C2C2C2)]
    rx_addr_p1: u64,
}

impl_address_register!(
    RxAddrP1,
    RxAddrP1Fields,
    0x0B,
    rx_addr_p1,
    with_rx_addr_p1,
    "RX address data pipe 1. Default value: `0xC2C2C2C2C2`.",
    [3, 4, 5]
);

/// # RX_ADDR_P2 register
/// RX address data pipe 2. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0C`
///
/// ## Fields
/// #### `rx_addr_p2` | bits 7:0
/// RX address data pipe 2. Default value: `0xC3`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxAddrP2::new();
/// assert_eq!(reg.into_bits(), 0xC3);
///
/// // Write fields
/// let reg = RxAddrP2::new().with_rx_addr_p2(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP2 {
    /// RX address data pipe 2. Default value: `0xC3`.
    #[bits(8, default = 0xC3)]
    pub rx_addr_p2: u8,
}
impl_register!(RxAddrP2, 0x0C);

/// # RX_ADDR_P3 register
/// RX address data pipe 3. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0D`
///
/// ## Fields
/// #### `rx_addr_p3` | bits 7:0
/// RX address data pipe 3. Default value: `0xC4`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxAddrP3::new();
/// assert_eq!(reg.into_bits(), 0xC4);
///
/// // Write fields
/// let reg = RxAddrP3::new().with_rx_addr_p3(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP3 {
    /// RX address data pipe 3. Default value: `0xC4`.
    #[bits(8, default = 0xC4)]
    pub rx_addr_p3: u8,
}
impl_register!(RxAddrP3, 0x0D);

/// # RX_ADDR_P4 register
/// RX address data pipe 4. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0E`
///
/// ## Fields
/// #### `rx_addr_p4` | bits 7:0
/// RX address data pipe 4. Default value: `0xC5`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxAddrP4::new();
/// assert_eq!(reg.into_bits(), 0xC5);
///
/// // Write fields
/// let reg = RxAddrP4::new().with_rx_addr_p4(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP4 {
    /// RX address data pipe 4. Default value: `0xC5`.
    #[bits(8, default = 0xC5)]
    pub rx_addr_p4: u8,
}
impl_register!(RxAddrP4, 0x0E);

/// # RX_ADDR_P5 register
/// RX address data pipe 5. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0F`
///
/// ## Fields
/// #### `rx_addr_p5` | bits 7:0
/// RX address data pipe 5. Default value: `0xC6`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxAddrP5::new();
/// assert_eq!(reg.into_bits(), 0xC6);
///
/// // Write fields
/// let reg = RxAddrP5::new().with_rx_addr_p5(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP5 {
    /// RX address data pipe 5. Default value: `0xC6`.
    #[bits(8, default = 0xC6)]
    pub rx_addr_p5: u8,
}
impl_register!(RxAddrP5, 0x0F);

/// # TX_ADDR register
/// TX address. Set [`RxAddrP0`] equal to this address to handle ACK automatically.
///
/// Address = `0x10`
///
/// Const parameter `N`: address width in bytes.
/// <div class="warning">
/// N must be of {3, 4, 5}.
/// </div>
///
/// ## Fields
/// #### `tx_addr` | bits 39:0
/// TX address. Default value: `0xE7E7E7E7E7`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = TxAddr::<4>::new();
/// assert_eq!(reg.tx_addr(), 0xE7E7E7E7E7);
///
/// // Write fields
/// let reg = TxAddr::<5>::new().with_tx_addr(0xC2840DF659);
/// assert_eq!(reg.tx_addr(), 0xC2840DF659);
///
/// // Generate write register bytes
/// assert_eq!(reg.as_write_bytes(), [0x10 | 0b0010_0000, 0x59, 0xF6, 0x0D, 0x84, 0xC2]);
///
/// // For 3 byte address width
/// let reg = TxAddr::<3>::new().with_tx_addr(0xC2840DF659);
/// assert_eq!(reg.as_write_bytes(), [0x10 | 0b0010_0000, 0x59, 0xF6, 0x0D]);
/// ```
#[derive(Copy, Clone)]
pub struct TxAddr<const N: usize>(TxAddrFields);

#[bitfield(u64, order = Msb)]
struct TxAddrFields {
    #[bits(24)]
    __: u32,

    /// TX address. Default value: `0xE7E7E7E7E7`.
    #[bits(40, default = 0xE7E7E7E7E7)]
    tx_addr: u64,
}

impl_address_register!(
    TxAddr,
    TxAddrFields,
    0x10,
    tx_addr,
    with_tx_addr,
    "TX address. Default value: `0xE7E7E7E7E7`.",
    [3, 4, 5]
);

/// # RX_PW_P0 register
/// RX payload width for data pipe 0.
///
/// Address = `0x11`
///
/// ## Fields
/// #### `rx_pw_p0` | bits 7:0
/// RX payload width for data pipe 0. 1 - 32 bytes. `0` = pipe not used.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxPwP0::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = RxPwP0::new().with_rx_pw_p0(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RxPwP0 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 0. 1 - 32 bytes. `0` = pipe not used.
    #[bits(6)]
    pub rx_pw_p0: u8,
}
impl_register!(RxPwP0, 0x11);

/// # RX_PW_P1 register
/// RX payload width for data pipe 1.
///
/// Address = `0x12`
///
/// ## Fields
/// #### `rx_pw_p1` | bits 7:0
/// RX payload width for data pipe 1. 1 - 32 bytes. `0` = pipe not used.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxPwP1::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = RxPwP1::new().with_rx_pw_p1(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RxPwP1 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 1. 1 - 32 bytes. `0` = pipe not used.
    #[bits(6)]
    pub rx_pw_p1: u8,
}
impl_register!(RxPwP1, 0x12);

/// # RX_PW_P2 register
/// RX payload width for data pipe 2.
///
/// Address = `0x13`
///
/// ## Fields
/// #### `rx_pw_p2` | bits 7:0
/// RX payload width for data pipe 2. 1 - 32 bytes. `0` = pipe not used.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxPwP2::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = RxPwP2::new().with_rx_pw_p2(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RxPwP2 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 2. 1 - 32 bytes. `0` = pipe not used.
    #[bits(6)]
    pub rx_pw_p2: u8,
}
impl_register!(RxPwP2, 0x13);

/// # RX_PW_P3 register
/// RX payload width for data pipe 3.
///
/// Address = `0x14`
///
/// ## Fields
/// #### `rx_pw_p3` | bits 7:0
/// RX payload width for data pipe 3. 1 - 32 bytes. `0` = pipe not used.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxPwP3::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = RxPwP3::new().with_rx_pw_p3(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RxPwP3 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 3. 1 - 32 bytes. `0` = pipe not used.
    #[bits(6)]
    pub rx_pw_p3: u8,
}
impl_register!(RxPwP3, 0x14);

/// # RX_PW_P4 register
/// RX payload width for data pipe 4.
///
/// Address = `0x15`
///
/// ## Fields
/// #### `rx_pw_p4` | bits 7:0
/// RX payload width for data pipe 4. 1 - 32 bytes. `0` = pipe not used.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxPwP4::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = RxPwP4::new().with_rx_pw_p4(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RxPwP4 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 4. 1 - 32 bytes. `0` = pipe not used.
    #[bits(6)]
    pub rx_pw_p4: u8,
}
impl_register!(RxPwP4, 0x15);

/// # RX_PW_P5 register
/// RX payload width for data pipe 5.
///
/// Address = `0x16`
///
/// ## Fields
/// #### `rx_pw_p5` | bits 7:0
/// RX payload width for data pipe 5. 1 - 32 bytes. `0` = pipe not used.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = RxPwP5::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = RxPwP5::new().with_rx_pw_p5(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RxPwP5 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 5. 1 - 32 bytes. `0` = pipe not used.
    #[bits(6)]
    pub rx_pw_p5: u8,
}
impl_register!(RxPwP5, 0x16);

/// # FIFO_STATUS register
/// Status of TX/RX FIFOs.
///
/// Address = `0x17`
///
/// ## Fields
/// All fields are read-only.
///
/// #### `tx_reuse` | bit 6
/// Reuse last transmitted data packet if set high.
/// The packet is repeatedly retransmitted as long as CE is high.
/// TX_REUSE is set by the [`REUSE_TX_PL`][crate::ReuseTxPl] command and reset by
/// [`W_TX_PAYLOAD`][crate::WTxPayload] or [`FLUSH_TX`][crate::FlushTx].
///
/// #### `tx_full` | bit 5
/// TX FIFO full flag.
///
/// #### `tx_empty` | bit 4
/// TX FIFO empty flag.
///
/// #### `rx_full` | bit 1
/// RX FIFO full flag.
///
/// #### `rx_empty` | bit 0
/// RX FIFO empty flag.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = FifoStatus::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Read fields
/// let reg = FifoStatus::from_bits(0b0010_0010);
/// assert!(!reg.tx_reuse());
/// assert!(reg.tx_full());
/// assert!(!reg.tx_empty());
/// assert!(reg.rx_full());
/// assert!(!reg.rx_empty());
/// ```
#[bitfield(u8, order = Msb)]
pub struct FifoStatus {
    #[bits(1)]
    __: bool,

    /// Reuse last transmitted data packet if set high.
    /// The packet is repeatedly retransmitted as long as CE is high.
    /// TX_REUSE is set by the REUSE_TX_PL command and reset by
    /// W_TX_PAYLOAD or FLUSH_TX.
    #[bits(1, access = RO)]
    pub tx_reuse: bool,

    /// TX FIFO full flag.
    #[bits(1, access = RO)]
    pub tx_full: bool,

    /// TX FIFO empty flag.
    #[bits(1, access = RO)]
    pub tx_empty: bool,

    #[bits(2)]
    __: u8,

    /// RX FIFO full flag.
    #[bits(1, access = RO)]
    pub rx_full: bool,

    /// RX FIFO empty flag.
    #[bits(1, access = RO)]
    pub rx_empty: bool,
}
impl_register!(FifoStatus, 0x17);

/// # DYNPD register
/// Enable dynamic payload length for data pipes 0-5.
///
/// Address = `0x1C`
///
/// ## Fields
///
/// #### `dpl_pN` | bit `N`
/// Enable dynamic payload length on data pipes `N` = 0-5.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = Dynpd::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = Dynpd::new()
///     .with_dpl_p5(true)
///     .with_dpl_p4(false)
///     .with_dpl_p3(false)
///     .with_dpl_p2(false)
///     .with_dpl_p1(true)
///     .with_dpl_p0(false);
/// assert_eq!(reg.into_bits(), 0b0010_0010);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Dynpd {
    #[bits(2)]
    __: u8,

    /// Enable dynamic payload length for data pipe 5.
    /// Requires `en_dpl` in [`Feature`] and `enaa_p5` in [`EnAa`].
    #[bits(1)]
    pub dpl_p5: bool,
    /// Enable dynamic payload length for data pipe 4.
    /// Requires `en_dpl` in [`Feature`] and `enaa_p4` in [`EnAa`].
    #[bits(1)]
    pub dpl_p4: bool,
    /// Enable dynamic payload length for data pipe 3.
    /// Requires `en_dpl` in [`Feature`] and `enaa_p3` in [`EnAa`].
    #[bits(1)]
    pub dpl_p3: bool,
    /// Enable dynamic payload length for data pipe 2.
    /// Requires `en_dpl` in [`Feature`] and `enaa_p2` in [`EnAa`].
    #[bits(1)]
    pub dpl_p2: bool,
    /// Enable dynamic payload length for data pipe 1.
    /// Requires `en_dpl` in [`Feature`] and `enaa_p1` in [`EnAa`].
    #[bits(1)]
    pub dpl_p1: bool,
    /// Enable dynamic payload length for data pipe 0.
    /// Requires `en_dpl` in [`Feature`] and `enaa_p0` in [`EnAa`].
    #[bits(1)]
    pub dpl_p0: bool,
}
impl_register!(Dynpd, 0x1C);

/// # FEATURE register
/// Enable features _Dynamic Payload Length_, _Payload with ACK_ and `W_TX_PAYLOAD_NO_ACK` command.
///
/// Address = `0x01D`
///
/// ## Fields
/// #### `en_dpl` | bit 2
/// Enables _Dynamic Payload Length_.
///
/// #### `en_ack_pay` | bit 1
/// Enables _Payload with ACK_.
///
/// #### `en_dyn_ack` | bit 0
/// Enables `W_TX_PAYLOAD_NO_ACK` command.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers::*;
///
/// // Default value
/// let reg = Feature::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = Feature::new()
///     .with_en_dpl(false)
///     .with_en_ack_pay(true)
///     .with_en_dyn_ack(true);
/// assert_eq!(reg.into_bits(), 0b0000_0011);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Feature {
    #[bits(5)]
    __: u8,

    /// Enables _Dynamic Payload Length_.
    #[bits(1)]
    pub en_dpl: bool,

    /// Enables _Payload with ACK_.
    #[bits(1)]
    pub en_ack_pay: bool,

    /// Enables 'W_TX_PAYLOAD_NO_ACK' command.
    #[bits(1)]
    pub en_dyn_ack: bool,
}
impl_register!(Feature, 0x1D);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_config() {
        // Check default
        let reg = Config::new();
        assert_eq!(reg.into_bits(), 0b0000_1000);
        // Check fields
        let reg = reg
            .with_crco(Crco::TwoByte)
            .with_en_crc(false)
            .with_mask_max_rt(true)
            .with_mask_tx_ds(true)
            .with_mask_rx_dr(true);
        assert_eq!(reg.into_bits(), 0b0111_0100);
        // Check read command
        let read_reg_bytes = Config::as_read_bytes();
        assert_eq!(read_reg_bytes, [0x00, 0]);
        // Check write command
        let write_reg_bytes = reg.as_write_bytes();
        assert_eq!(write_reg_bytes, [0b0010_0000 | 0x00, 0b0111_0100]);
    }

    #[test]
    fn test_reg_rf_ch() {
        // Check default
        let reg = RfCh::new();
        assert_eq!(reg.into_bits(), 0b0000_0010);
        // Check fields
        let reg = reg.with_rf_ch(48);
        assert_eq!(reg.into_bits(), 48);
        // Check read command
        let read_reg_bytes = RfCh::as_read_bytes();
        assert_eq!(read_reg_bytes, [0x05, 0]);
        // Check write command
        let write_reg_bytes = reg.as_write_bytes();
        assert_eq!(write_reg_bytes, [0b0010_0000 | 0x05, 48]);
    }

    #[test]
    fn test_reg_status() {
        // Check default
        let reg = Status::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let mut reg = Status::from_bits(0b0010_0110);
        assert!(!reg.tx_full());
        assert_eq!(reg.rx_p_no(), RxPipeNo::Pipe3);
        assert!(!reg.max_rt());
        assert!(reg.tx_ds());
        assert!(!reg.rx_dr());
        // Set field
        reg.set_max_rt(true);
        assert_eq!(reg.into_bits(), 0b0011_0110);
        // Check read command
        let read_reg_bytes = Status::as_read_bytes();
        assert_eq!(read_reg_bytes, [0x07, 0]);
        // Check write command
        let write_reg_bytes = reg.as_write_bytes();
        assert_eq!(write_reg_bytes, [0b0010_0000 | 0x07, 0b0011_0110]);
    }

    #[test]
    fn test_reg_cd() {
        // Check default
        let reg = Rpd::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = Rpd::from_bits(1);
        assert_eq!(reg.into_bits(), 1);
        // Check read command
        let read_reg_bytes = Rpd::as_read_bytes();
        assert_eq!(read_reg_bytes, [0x09, 0]);
    }

    #[test]
    fn test_reg_rx_addr_p0() {
        // Check default
        const RX_ADDR_P0_WIDTH3: RxAddrP0<3> = RxAddrP0::<3>::new();
        assert_eq!(RX_ADDR_P0_WIDTH3.rx_addr_p0(), 0xE7E7E7E7E7);

        // Check write reg bytes
        const RX_ADDR_P0_WIDTH3_COPY: RxAddrP0<3> = RX_ADDR_P0_WIDTH3.with_rx_addr_p0(0x8106310AC0);
        assert_eq!(
            RX_ADDR_P0_WIDTH3_COPY.as_write_bytes(),
            [0b0010_0000 | 0x0A, 0xC0, 0x0A, 0x31]
        );

        // Check field
        assert_eq!(RX_ADDR_P0_WIDTH3_COPY.rx_addr_p0(), 0x8106310AC0);
        // Check as_read_bytes
        assert_eq!(RxAddrP0::<4>::as_read_bytes(), [0x0A, 0, 0, 0, 0]);

        // Check 5 byte width
        const RX_ADDR_P0_WIDTH5: RxAddrP0<5> = RxAddrP0::from_bits(0x605F4459BF);
        let write_reg_bytes = RX_ADDR_P0_WIDTH5.as_write_bytes();
        assert_eq!(
            write_reg_bytes,
            [0b0010_0000 | 0x0A, 0xBF, 0x59, 0x44, 0x5F, 0x60]
        );
    }

    #[test]
    fn test_reg_rx_addr_p1() {
        // Check default
        const RX_ADDR_P1_WIDTH3: RxAddrP1<3> = RxAddrP1::<3>::new();
        assert_eq!(RX_ADDR_P1_WIDTH3.rx_addr_p1(), 0xC2C2C2C2C2);

        // Check write reg bytes
        const RX_ADDR_P1_WIDTH3_COPY: RxAddrP1<3> = RX_ADDR_P1_WIDTH3.with_rx_addr_p1(0x0144DF0AEC);
        assert_eq!(
            RX_ADDR_P1_WIDTH3_COPY.as_write_bytes(),
            [0b0010_0000 | 0x0B, 0xEC, 0x0A, 0xDF]
        );

        // Check field
        assert_eq!(RX_ADDR_P1_WIDTH3_COPY.rx_addr_p1(), 0x0144DF0AEC);
        // Check as_read_bytes
        assert_eq!(RxAddrP1::<4>::as_read_bytes(), [0x0B, 0, 0, 0, 0]);

        // Check 5 byte width
        const RX_ADDR_P1_WIDTH5: RxAddrP1<5> = RxAddrP1::from_bits(0xFF32C8ED07);
        let write_reg_bytes = RX_ADDR_P1_WIDTH5.as_write_bytes();
        assert_eq!(
            write_reg_bytes,
            [0b0010_0000 | 0x0B, 0x07, 0xED, 0xC8, 0x32, 0xFF]
        );
    }

    #[test]
    fn test_reg_tx_addr() {
        // Check default
        const TX_ADDR_WIDTH4: TxAddr<4> = TxAddr::new();
        assert_eq!(TX_ADDR_WIDTH4.tx_addr(), 0xE7E7E7E7E7);

        // Check write reg bytes
        const TX_ADDR_WIDTH4_COPY: TxAddr<4> = TX_ADDR_WIDTH4.with_tx_addr(0x17E73A6C58);
        assert_eq!(
            TX_ADDR_WIDTH4_COPY.as_write_bytes(),
            [0b0010_0000 | 0x10, 0x58, 0x6C, 0x3A, 0xE7]
        );

        // Check field
        assert_eq!(TX_ADDR_WIDTH4_COPY.tx_addr(), 0x17E73A6C58);
        // Check as_read_bytes
        assert_eq!(TxAddr::<4>::as_read_bytes(), [0x10, 0, 0, 0, 0]);

        // Check 5 byte width
        const TX_ADDR_WIDTH5: TxAddr<5> = TxAddr::from_bits(0x170F431EDC);
        let write_reg_bytes = TX_ADDR_WIDTH5.as_write_bytes();
        assert_eq!(
            write_reg_bytes,
            [0b0010_0000 | 0x10, 0xDC, 0x1E, 0x43, 0x0F, 0x17]
        );
    }

    #[test]
    fn test_reg_rx_pw_p0() {
        // Check default
        let reg = RxPwP0::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = reg.with_rx_pw_p0(32);
        assert_eq!(reg.into_bits(), 32);
        // Check read command
        let read_reg_bytes = RxPwP0::as_read_bytes();
        assert_eq!(read_reg_bytes, [0x11, 0]);
        // Check write command
        let write_reg_bytes = reg.as_write_bytes();
        assert_eq!(write_reg_bytes, [0b0010_0000 | 0x11, 32]);
    }

    #[test]
    fn test_reg_fifo_status() {
        // Check default
        let reg = FifoStatus::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = FifoStatus::from_bits(0b0100_0001);
        assert!(reg.rx_empty());
        assert!(!reg.rx_full());
        assert!(!reg.tx_empty());
        assert!(!reg.tx_full());
        assert!(reg.tx_reuse());
        // Check read command
        let read_reg_bytes = FifoStatus::as_read_bytes();
        assert_eq!(read_reg_bytes, [0x17, 0]);
    }

    #[test]
    fn test_reg_feature() {
        // Check default
        let reg = Feature::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = reg
            .with_en_dyn_ack(true)
            .with_en_ack_pay(true)
            .with_en_dpl(false);
        assert_eq!(reg.into_bits(), 0b0000_0011);
        // Check read command
        let read_reg_bytes = Feature::as_read_bytes();
        assert_eq!(read_reg_bytes, [0x1D, 0]);
        // Check write command
        let write_reg_bytes = reg.as_write_bytes();
        assert_eq!(write_reg_bytes, [0b0010_0000 | 0x1D, 0b0000_0011]);
    }
}
