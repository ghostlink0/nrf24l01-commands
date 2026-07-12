//! Enums for certain nRF24L01+ register fields.

/// Implement `into_bits()` and `from_bits()` for enums - this is required to work with `bitfield-struct`.
/// `$bitmask` is the maximum value the enum could be, the enum must be defined for every value up to this value.
/// This ensures `from_bits()` can be safely passed any `u8` value.
macro_rules! impl_bitfield_enum {
    ($type:ty, $bitmask:literal) => {
        impl $type {
            pub const fn into_bits(self) -> u8 {
                self as _
            }
            pub const fn from_bits(bits: u8) -> Self {
                #[allow(clippy::macro_metavars_in_unsafe)]
                unsafe {
                    core::mem::transmute(bits & $bitmask)
                }
            }
        }
    };
}

/// CRC encoding scheme.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crco {
    /// 1 byte CRC
    OneByte = 0,
    /// 2 byte CRC
    TwoByte = 1,
}
impl_bitfield_enum!(Crco, 1);

/// RX/TX address field width in bytes.
/// LSByte is used if address width is below 5 bytes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressWidth {
    Illegal = 0,
    ThreeByte = 0b01,
    FourByte = 0b10,
    FiveByte = 0b11,
}
impl_bitfield_enum!(AddressWidth, 0b11);

/// Auto retransmit delay.
///
/// `0000`: Wait 250µS
///
/// `0001`: Wait 500µS
///
/// `0010`: Wait 750µS
///
/// ……
///
/// `1111`: Wait 4000µS
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AutoRetransmitDelay {
    US250 = 0b0000,
    US500 = 0b0001,
    US750 = 0b0010,
    US1000 = 0b0011,
    US1250 = 0b0100,
    US1500 = 0b0101,
    US1750 = 0b0110,
    US2000 = 0b0111,
    US2250 = 0b1000,
    US2500 = 0b1001,
    US2750 = 0b1010,
    US3000 = 0b1011,
    US3250 = 0b1100,
    US3500 = 0b1101,
    US3750 = 0b1110,
    US4000 = 0b1111,
}
impl_bitfield_enum!(AutoRetransmitDelay, 0b1111);

/// High speed data rate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RfDrHigh {
    Mbps1 = 0,
    Mbps2 = 1,
}
impl_bitfield_enum!(RfDrHigh, 1);

/// Set RF output power in TX mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RfPower {
    /// -18 dBm
    Neg18Dbm = 0b00,
    /// -12 dBm
    Neg12Dbm = 0b01,
    /// -6 dBm
    Neg6Dbm = 0b10,
    /// 0 dBm
    Dbm0 = 0b11,
}
impl_bitfield_enum!(RfPower, 0b11);

/// Data pipe number for the payload available from reading RX FIFO.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxPipeNo {
    Pipe0 = 0,
    Pipe1 = 1,
    Pipe2 = 2,
    Pipe3 = 3,
    Pipe4 = 4,
    Pipe5 = 5,
    NotUsed = 0b110,
    RxFifoEmpty = 0b111,
}
impl_bitfield_enum!(RxPipeNo, 0b111);
