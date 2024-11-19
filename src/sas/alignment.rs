#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Alignment {
    value: u8,
}

impl Alignment {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Alignment { value }),
            0x01 => Some(Alignment { value }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_alignment_struct() {
        let alignment1 = Alignment { value: 0x00 };
        let alignment2 = Alignment::from_u8(0x00).unwrap();

        assert_eq!(alignment1, alignment2);
    }

    #[test]
    fn test_alignment_from_u8() {
        let alignment = Alignment::from_u8(0x00);
        assert_eq!(alignment, Some(Alignment { value: 0x00 }));

        let alignment = Alignment::from_u8(0x01);
        assert_eq!(alignment, Some(Alignment { value: 0x01 }));

        let alignment = Alignment::from_u8(0x02);
        assert_eq!(alignment, None);
    }
}
