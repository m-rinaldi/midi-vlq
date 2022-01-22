use local_vec::CopyLocalVec;

mod from;
mod ux;
mod as_ref;
mod eq;

type VlqBuf = CopyLocalVec<u8, 4>; 

// Test the README file as well
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;

/// A VLQ-encoded integer for the Standard MIDI file format
/// # Examples
/// ```
/// use midi_vlq::MidiVlq;
/// // encode 0x4000 as VLQ
/// let vlq = MidiVlq::from(0x4000_u16);
/// assert_eq!(vlq, [0b10000001, 0b10000000, 0]);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct MidiVlq(VlqBuf);

impl MidiVlq {
    /// length in bytes
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_u32(&self) -> u32 {
        <Self as Into<u32>>::into(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::MidiVlq;
    #[test]
    fn it_works() {
        let vlq = MidiVlq::from(0u8);
        assert_eq!(vlq, [0]);
        assert_eq!(vlq.to_u32(), 0);

        let vlq = MidiVlq::from(0x40u8);
        assert_eq!(vlq, [0x40]);
        assert_eq!(vlq.to_u32(), 0x40);

        let vlq = MidiVlq::from(0x7fu8);
        assert_eq!(vlq, [0x7f]);
        assert_eq!(vlq.to_u32(), 0x7f);

        let vlq = MidiVlq::from(0x80u8);
        assert_eq!(vlq, [0x81, 0x00]);
        assert_eq!(vlq.to_u32(), 0x80);

        let vlq = MidiVlq::from(0x2000u16);
        assert_eq!(vlq, [0xC0, 0x00]);
        assert_eq!(vlq.to_u32(), 0x2000);

        let vlq = MidiVlq::from(0x3fffu16);
        assert_eq!(vlq, [0xff, 0x7f]);
        assert_eq!(vlq.to_u32(), 0x3fff);

        let vlq = MidiVlq::from(0x4000u16);
        assert_eq!(vlq, [0x81, 0x80, 0x00]);
        assert_eq!(vlq.to_u32(), 0x4000);

        let vlq = MidiVlq::try_from(0x10_0000u32);
        assert!(vlq.is_ok());
        let vlq = vlq.unwrap();
        assert_eq!(vlq, [0xc0, 0x80, 0x00]);
        assert_eq!(vlq.to_u32(), 0x10_0000);

        let vlq = MidiVlq::try_from(0x1f_ffffu32);
        assert!(vlq.is_ok());
        let vlq = vlq.unwrap();
        assert_eq!(vlq, [0xff, 0xff, 0x7f]);
        assert_eq!(vlq.to_u32(), 0x1f_ffff);

        let vlq = MidiVlq::try_from(0x20_0000u32);
        assert!(vlq.is_ok());
        let vlq = vlq.unwrap();
        assert_eq!(vlq, [0x81, 0x80, 0x80, 0x00]);
        assert_eq!(vlq.to_u32(), 0x20_0000);

        let vlq = MidiVlq::try_from(0x800_0000u32);
        assert!(vlq.is_ok());
        let vlq = vlq.unwrap();
        assert_eq!(vlq, [0xc0, 0x80, 0x80, 0x00]);
        assert_eq!(vlq.to_u32(), 0x800_0000);

        let vlq = MidiVlq::try_from(0x0fff_ffffu32);
        assert!(vlq.is_ok());
        let vlq = vlq.unwrap();
        assert_eq!(vlq, [0xff, 0xff, 0xff, 0x7f]);
        assert_eq!(vlq.to_u32(), 0x0fff_ffff);
    }
}
