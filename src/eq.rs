use crate::MidiVlq;

impl PartialEq<Self> for MidiVlq {
    fn eq(&self, other: &MidiVlq) -> bool {
        let lhs = self.to_u32();
        let rhs = other.to_u32();
        lhs == rhs
    }
}

impl Eq for MidiVlq {}

impl PartialEq<[u8]> for MidiVlq {
    fn eq(&self, other: &[u8]) -> bool {
        // do not convert other into u32 as we don't know whether it has a proper VLQ format
        self.as_ref() == other
    }
}

// to take advantage of arrays [u8; N] automatically coercing into slices [u8]
fn eq(lhs: &MidiVlq, rhs: &[u8]) -> bool {
    lhs == rhs
}


// TODO use a macro to avoid repetition
impl PartialEq<[u8; 1]> for MidiVlq {
    fn eq(&self, other: &[u8; 1]) -> bool {
        eq(self, other)
    }
}

impl PartialEq<[u8; 2]> for MidiVlq {
    fn eq(&self, other: &[u8; 2]) -> bool {
        eq(self, other)
    }
}

impl PartialEq<[u8; 3]> for MidiVlq {
    fn eq(&self, other: &[u8; 3]) -> bool {
        eq(self, other)
    }
}

impl PartialEq<[u8; 4]> for MidiVlq {
    fn eq(&self, other: &[u8; 4]) -> bool {
        eq(self, other)
    }
}