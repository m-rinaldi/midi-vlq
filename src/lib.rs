use local_vec::LocalVec;

mod from;
mod u7;
mod as_ref;

type VlqBuf = LocalVec<u8, 4>;

pub struct MidiVlq(VlqBuf);

impl MidiVlq {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/*
    VLQ -> u32 is infallible (==> u32::From<u32> & VLQ::Into<u32>)
    
*/

// TODO same for u16
/* 
impl From<u8> for MidiVlq 
impl From<u16> for MidiVlq
impl TryFrom<u32> for MidiVlq

impl Into<u32> for MidiVlq

impl Deref<Target=[u8]> for MidiVlq ???
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
