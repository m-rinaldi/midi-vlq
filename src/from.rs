use crate::MidiVlq;

use super::{VlqBuf, u7::U7};

fn encode_ending_byte(val: U7) -> u8 {
    val.into()
}

fn encode_nonending_byte(val: U7) -> u8 {
    0b1000_0000 | Into::<u8>::into(val)
}

impl From<u8> for MidiVlq {
    fn from(val: u8) -> Self {
        if val <= 127 {
            return MidiVlq(VlqBuf::from_array([val]));
        }

        debug_assert!(0b1000_0000 & val != 0);
        let msb = encode_nonending_byte(U7::try_from(1).unwrap());
        let lsb = encode_ending_byte(U7::try_from(0b0111_1111 & val).unwrap());
        MidiVlq(VlqBuf::from_array([msb, lsb]))
    }
}

impl From<MidiVlq> for u32 {
    fn from(_vlq: MidiVlq) -> u32 {
        todo!()
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
        assert_eq!(vlq.as_ref(), [0b1000_0001, 0]);

        let vlq = MidiVlq::from(255_u8);
        assert_eq!(vlq.len(), 2);
        assert_eq!(vlq.as_ref(), [0b1000_0001, 0b0111_1111]);
    }
}