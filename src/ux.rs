// 7-bit unsigned integer
pub struct U7(u8);
pub struct U14(u16);
pub struct U21(u32);
pub struct U28(u32);

// TODO write a macro instead
// TODO trait instead (e.g., MaxBound)?
impl U7 {
    pub const MAX: u32 = (1 << 7) - 1;
}

impl U14 {
    pub const MAX: u32 = (1 << 14) - 1;
}

impl U21 {
    pub const MAX: u32 = (1 << 21) - 1;
}

impl U28 {
    pub const MAX: u32 = (1 << 28) - 1;
}

impl TryFrom<u8> for U7 {
    type Error = ();

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        if val as u32 <= U7::MAX {
            return Ok(U7(val));
        }

        Err(())
    }
}

impl TryFrom<u16> for U14 {
    type Error = ();

    fn try_from(val: u16) -> Result<Self, Self::Error> {
        if val as u32 <= U14::MAX {
            return Ok(U14(val))
        }

        Err(())
    }
}

impl TryFrom<u32> for U21 {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if val <= U21::MAX {
            return Ok(U21(val))
        }

        Err(())
    }
}

impl TryFrom<u32> for U28 {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if val <= U28::MAX {
            return Ok(U28(val))
        }

        Err(())
    }
}

impl From<U7> for u8 {
    fn from(val: U7) -> u8 {
        val.0
    }
}

impl From<U14> for u16 {
    fn from(val: U14) -> u16 {
        val.0
    }
}

impl From<U21> for u32 {
    fn from(val: U21) -> u32 {
        val.0
    }
}

impl From<U28> for u32 {
    fn from(val: U28) -> u32 {
        val.0 as u32
    }
}