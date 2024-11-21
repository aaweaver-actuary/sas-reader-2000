/// Represents the endianness of the data in the SAS file.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Endianness {
    Big,
    Little,
}

impl Endianness {
    pub fn from_u8(value: u8) -> Result<Self, String> {
        match value {
            0x00 => Ok(Endianness::Big),
            0x01 => Ok(Endianness::Little),
            _ => {
                let message = format!(
                    "Unknown endianness code from binary: {}. Expected either 0 for Big or 1 for Little.",
                    value
                );
                Err(message)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endianness_from_u8_when_valid() {
        assert_eq!(Endianness::from_u8(0x00).unwrap(), Endianness::Big);
        assert_eq!(Endianness::from_u8(0x01).unwrap(), Endianness::Little);
    }

    #[test]
    #[should_panic(
        expected = "Unknown endianness code from binary: 2. Expected either 0 for Big or 1 for Little."
    )]
    fn test_endianness_from_u8_panics_when_invalid_code_unwrapped() {
        Endianness::from_u8(2).unwrap();
    }

}
