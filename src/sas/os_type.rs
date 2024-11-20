#[derive(Debug, PartialEq)]
pub enum OsType {
    Windows,
    Unix,
}

impl OsType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => OsType::Unix,
            2 => OsType::Windows,
            _ => panic!("Unknown OS type code from binary: {}. Expected either 1 for Unix or 2 for Windows.", value),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_make_os_type_enum() {
        let os_type1 = OsType::Unix;
        let os_type2 = OsType::from_u8(1);

        assert_eq!(os_type1, os_type2);
    }

    #[test]
    fn test_os_type_from_u8_when_unix() {
        let os_type = OsType::from_u8(1);
        assert_eq!(os_type, OsType::Unix);
    }

    #[test]
    fn test_os_type_from_u8_when_windows() {
        let os_type = OsType::from_u8(2);
        assert_eq!(os_type, OsType::Windows);
    }

    #[test]
    #[should_panic(
        expected = "Unknown OS type code from binary: 3. Expected either 1 for Unix or 2 for Windows."
    )]
    fn test_os_type_from_u8_when_unknown() {
        OsType::from_u8(3);
    }
}
