#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Alignment {
    pub value: u8,
}

impl Alignment {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x33 => Alignment { value: 4 },
            _ => Alignment { value: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_alignment_struct() {
        let alignment1 = Alignment { value: 0x00 };
        let alignment2 = Alignment::from_u8(0x00);

        assert_eq!(alignment1, alignment2);
    }

    #[test]
    fn test_alignment_from_u8() {
        let alignment = Alignment::from_u8(0x00);
        assert_eq!(alignment, Alignment { value: 0x00 });

        let alignment = Alignment::from_u8(0x01);
        assert_eq!(alignment, Alignment { value: 0x00 });

        let alignment = Alignment::from_u8(0x02);
        assert_eq!(alignment, Alignment { value: 0x00 });

        let alignment = Alignment::from_u8(0x04);
        assert_eq!(alignment, Alignment { value: 0x00 });

        let alignment = Alignment::from_u8(0x08);
        assert_eq!(alignment, Alignment { value: 0x00 });

        let alignment = Alignment::from_u8(0x33);
        assert_eq!(alignment, Alignment { value: 0x04 });
        assert_eq!(alignment, Alignment { value: 4 });
    }
}
