use local_vec::LocalVec;

mod from;
mod ux;
mod as_ref;

type VlqBuf = LocalVec<u8, 4>;

pub struct MidiVlq(VlqBuf);

impl MidiVlq {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
