// 7-bit unsigned integer
pub struct U7(u8);

impl TryFrom<u8> for U7 {
    type Error = ();

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        if 0b1000_0000 & val == 0 {
            return Ok(U7(val));
        }

        Err(())
    }
}

impl From<U7> for u8 {
    fn from(val: U7) -> u8 {
        val.0
    }
}