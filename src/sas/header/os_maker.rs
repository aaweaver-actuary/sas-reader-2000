#[derive(Debug, PartialEq)]
pub enum OsMaker {
    Sun,
    Ibm,
    Win,
}

impl OsMaker {
    pub fn from_ascii(value: String) -> Result<Self, String> {
        match value.as_str() {
            "SUN" => Ok(OsMaker::Sun),
            "IBM" => Ok(OsMaker::Ibm),
            "WIN" => Ok(OsMaker::Win),
            _ => panic!(
                "Unknown OS maker code from binary: {}. Expected either SUN, IBM, or WIN.",
                value
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_os_maker_enum() {
        let os_maker1 = OsMaker::Sun;
        let os_maker2 = OsMaker::from_ascii("SUN".to_string()).unwrap();

        assert_eq!(os_maker1, os_maker2);
    }

    #[test]
    fn test_os_maker_from_ascii_when_sun() {
        let os_maker = OsMaker::from_ascii("SUN".to_string()).unwrap();
        assert_eq!(os_maker, OsMaker::Sun);
    }

    #[test]
    fn test_os_maker_from_ascii_when_ibm() {
        let os_maker = OsMaker::from_ascii("IBM".to_string()).unwrap();
        assert_eq!(os_maker, OsMaker::Ibm);
    }

    #[test]
    fn test_os_maker_from_ascii_when_win() {
        let os_maker = OsMaker::from_ascii("WIN".to_string()).unwrap();
        assert_eq!(os_maker, OsMaker::Win);
    }

    #[test]
    #[should_panic(
        expected = "Unknown OS maker code from binary: MAC. Expected either SUN, IBM, or WIN."
    )]
    fn test_os_maker_from_ascii_when_unknown() {
        OsMaker::from_ascii("MAC".to_string()).unwrap();
    }
}
