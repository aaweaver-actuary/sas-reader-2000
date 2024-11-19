#[derive(Debug, PartialEq)]
pub enum PageType {
    Meta,
    Data,
    Mix,
    AMD,
    Unknown(u16),
}

impl PageType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => PageType::Meta,
            256 => PageType::Data,
            512 => PageType::Mix,
            1024 => PageType::AMD,
            _ => PageType::Unknown(value),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_page_type_from_u16() {
        assert_eq!(PageType::from_u16(0), PageType::Meta);
        assert_eq!(PageType::from_u16(256), PageType::Data);
        assert_eq!(PageType::from_u16(512), PageType::Mix);
        assert_eq!(PageType::from_u16(1024), PageType::AMD);
        assert_eq!(PageType::from_u16(1234), PageType::Unknown(1234));
    }
}
