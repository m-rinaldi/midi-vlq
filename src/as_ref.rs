use super::MidiVlq;

impl AsRef<[u8]> for MidiVlq {
    fn as_ref(&self) -> &[u8] {
        use std::ops::Deref;
        self.0.deref()
    }
}