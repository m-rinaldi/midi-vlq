use super::{MidiVlq, VlqBuf};
use super::ux::{U7, U14, U21, U28};

fn infallibly_encode(val: u16) -> MidiVlq {
    if val <= U7::MAX as u16 {
        // one byte
        MidiVlq(encode_u7(U7::try_from(val as u8).unwrap()))
    } else if val <= U14::MAX as u16 {
        // two bytes
        MidiVlq(encode_u14(U14::try_from(val as u16).unwrap()))
    } else {
        // three bytes
        MidiVlq(encode_u21(U21::try_from(val as u32).unwrap()))
    }
}

fn encode(val: u32) -> Option<MidiVlq> {
    if let Ok(val) = <u16 as TryFrom<u32>>::try_from(val) {
        // one or two bytes
        Some(infallibly_encode(val as u16))
    } else if val <= U21::MAX {
        // three bytes
        Some(MidiVlq(encode_u21(U21::try_from(val as u32).unwrap())))
    } else if val <= U28::MAX {
        // four bytes
        Some(MidiVlq(encode_u28(U28::try_from(val as u32).unwrap())))
    } else {
        // too large
        None
    }
}

fn encode_u7(val: U7) -> VlqBuf {
    let buf = VlqBuf::from_array([val.into()]);
    debug_assert_eq!(buf.len(), 1);
    return buf;
}

const FOLLOW_UP_BIT: u8 = 0b1000_0000;
const SEVEN_LOWEST_BITS: u8 = 0b0111_1111;

fn encode_u14(val: U14) -> VlqBuf {
    let val: u16 = val.into();

    let msb = (val >> 7) as u8 | FOLLOW_UP_BIT;
    let lsb = val as u8 & SEVEN_LOWEST_BITS;

    let buf = VlqBuf::from_array([msb, lsb]);
    debug_assert_eq!(buf.len(), 2);
    return buf;
}

fn encode_u21(val: U21) -> VlqBuf {
    let val: u32 = val.into();

    let b0 = (val >> 14) as u8 | FOLLOW_UP_BIT;
    let b1 = (val >>  7) as u8 | FOLLOW_UP_BIT;
    let b2 = val as u8 & SEVEN_LOWEST_BITS;

    let buf = VlqBuf::from_array([b0, b1, b2]);
    debug_assert_eq!(buf.len(), 3);
    return buf;
}

fn encode_u28(val: U28) -> VlqBuf {
    let val: u32 = val.into();

    let b0 = (val >> 21) as u8 | FOLLOW_UP_BIT;
    let b1 = (val >> 14) as u8 | FOLLOW_UP_BIT;
    let b2 = (val >>  7) as u8 | FOLLOW_UP_BIT;
    let b3 = val as u8 & SEVEN_LOWEST_BITS;

    let buf = VlqBuf::from_array([b0, b1, b2, b3]);
    debug_assert_eq!(buf.len(), 4);
    return buf;
}


impl From<u8> for MidiVlq {
    fn from(val: u8) -> Self {
        infallibly_encode(val as u16)
    }
}

impl From<u16> for MidiVlq {
    fn from(val: u16) -> Self {
        infallibly_encode(val)
    }
}

impl TryFrom<u32> for MidiVlq {
    type Error = String;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match encode(val) {
            None => Err("value too large".to_owned()),
            Some(vlq) => Ok(vlq),
        }
    }
}

impl From<MidiVlq> for u32 {
    fn from(vlq: MidiVlq) -> u32 {
        let slc = vlq.as_ref();
        debug_assert!(slc.len() <= 4);

        let mut val = 0u32;
        for byte in slc {
            val = (val << 7) | (0b0111_1111 & byte) as u32;
        }
        debug_assert_eq!(val & 0xf000_0000, 0);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_u8_single_byte() {
        let vlq = MidiVlq::from(0_u8);
        assert_eq!(vlq.len(), 1);
        assert_eq!(vlq.as_ref(), [0]);
        assert_eq!(vlq.to_u32(), 0u32);

        let vlq = MidiVlq::from(127_u8);
        assert_eq!(vlq.len(), 1);
        assert_eq!(vlq.as_ref(), [127]);

        let vlq = MidiVlq::from(1_u8);
        assert_eq!(vlq.len(), 1);
        assert_eq!(vlq.as_ref(), [1]);

        let vlq = MidiVlq::from(0x0f_u8);
        assert_eq!(vlq.len(), 1);
        assert_eq!(vlq.as_ref(), [0x0f]);
    }

    #[test]
    fn test_from_u8_two_bytes() {
        let vlq = MidiVlq::from(128_u8);
        assert_eq!(vlq.len(), 2);
        assert_eq!(vlq, [0b1000_0001, 0]);

        let vlq = MidiVlq::from(255_u8);
        assert_eq!(vlq.len(), 2);
        assert_eq!(vlq, [0b1000_0001, 0b0111_1111]);
    }

    #[test]
    fn test_from_u16_two_bytes() {
        let vlq = MidiVlq::from(256u16);
        assert_eq!(vlq.len(), 2);
        assert_eq!(vlq, [0b1000_0010, 0]);

        let vlq = MidiVlq::from(511u16);
        assert_eq!(vlq.len(), 2);
        assert_eq!(vlq, [0b1000_0011, 0b0111_1111]);

        let vlq = MidiVlq::from(16_383u16);
        assert_eq!(vlq.len(), 2);
        assert_eq!(vlq, [0b1111_1111, 0b0111_1111]);
    }

    #[test]
    fn test_from_u16_three_bytes() {
        let vlq = MidiVlq::from(16_384u16);
        assert_eq!(vlq.len(), 3);
    }

    #[test]
    fn test_from_u32_four_bytes() {
        let vlq = MidiVlq::try_from(268_435_454u32);
        matches!(vlq, Ok(_));
        let vlq = vlq.unwrap();
        assert_eq!(vlq.len(), 4);
    }

    #[test]
    fn test_from_u32_exceeded() {
        let vlq = MidiVlq::try_from(268_435_455u32);
        matches!(vlq, Err(_));
    }

    #[test]
    fn test_from_into_u32_max() {
        const MAX: u32 = 0x0fff_ffff;
        let vlq = MidiVlq::try_from(MAX);
        assert!(vlq.is_ok());
        let vlq = vlq.unwrap();
        assert_eq!(vlq.len(), 4);
        assert_eq!(vlq, [0xff, 0xff, 0xff, 0x7f]);
        assert_eq!(vlq.to_u32(), MAX);
    }
}