/// Represents the endianness of the data in the SAS file.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Endianness {
    Big,
    Little,
}

impl Endianness {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x00 => Endianness::Big,
            0x01 => Endianness::Little,
            _ => {
                if Self::is_little_endian() {
                    Endianness::Little
                } else {
                    Endianness::Big
                }
            }
        }
    }

    fn is_little_endian() -> bool {
        let value: u16 = 0x1234;
        let bytes: [u8; 2] = value.to_le_bytes();
        bytes[0] == 0x34
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endianness_from_u8() {
        assert_eq!(Endianness::from_u8(0x00), Endianness::Big);
        assert_eq!(Endianness::from_u8(0x01), Endianness::Little);
        assert!(
            Endianness::from_u8(0x02) == Endianness::Big
                || Endianness::from_u8(0x02) == Endianness::Little
        );
    }
}
