#[derive(Debug, PartialEq)]
pub enum OsType {
    Windows,
    Unix,
}

impl OsType {
    pub fn from_char(value: char) -> Result<Self, String> {
        match value {
            '1' => Ok(OsType::Unix),
            '2' => Ok(OsType::Windows),
            _ => panic!("Unknown OS type code from binary: {}. Expected either 1 for Unix or 2 for Windows.", value),
        }
    }

    pub fn from_u8(value: u8) -> Result<Self, String> {
        let ascii_value = value as char;
        match ascii_value {
            '1' => Ok(OsType::Unix),
            '2' => Ok(OsType::Windows),
            _ => panic!("Unknown OS type code from binary: {}. Expected either 1 for Unix or 2 for Windows.", ascii_value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_os_type_enum() {
        let os_type1 = OsType::Unix;
        let os_type2 = OsType::from_u8(b'1').unwrap();

        assert_eq!(os_type1, os_type2);
    }

    #[test]
    fn test_os_type_from_u8_when_unix() {
        let os_type = OsType::from_u8(b'1').unwrap();
        assert_eq!(os_type, OsType::Unix);

        let os_type = OsType::from_char('1').unwrap();
        assert_eq!(os_type, OsType::Unix);
    }

    #[test]
    fn test_os_type_from_u8_when_windows() {
        let os_type = OsType::from_u8(b'2').unwrap();
        assert_eq!(os_type, OsType::Windows);

        let os_type = OsType::from_char('2').unwrap();
        assert_eq!(os_type, OsType::Windows);
    }

    #[test]
    #[should_panic(
        expected = "Unknown OS type code from binary: 3. Expected either 1 for Unix or 2 for Windows."
    )]
    fn test_os_type_from_u8_when_unknown() {
        OsType::from_u8(b'3').unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Unknown OS type code from binary: 3. Expected either 1 for Unix or 2 for Windows."
    )]
    fn test_os_type_from_char_when_unknown() {
        OsType::from_char('3').unwrap();
    }
}
