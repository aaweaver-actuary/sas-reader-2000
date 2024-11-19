/// Represents the endianness of the data in the SAS file.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Endianness {
    Big,
    Little,
}

impl Endianness {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Endianness::Big),
            0x01 => Some(Endianness::Little),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endianness_from_u8() {
        assert_eq!(Endianness::from_u8(0x00), Some(Endianness::Big));
        assert_eq!(Endianness::from_u8(0x01), Some(Endianness::Little));
        assert_eq!(Endianness::from_u8(0x02), None);
    }
}
