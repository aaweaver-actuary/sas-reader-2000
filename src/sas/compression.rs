use crate::sas::errors::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Compression {
    None = 0,
    Truncated = 1,
    Rle = 4,
}

impl Compression {
    pub fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(Compression::None),
            1 => Ok(Compression::Truncated),
            4 => Ok(Compression::Rle),
            _ => Err(Error::InvalidCompression(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let compression1 = Compression::from_u8(0).unwrap();
        let compression2 = Compression::None;

        assert_eq!(compression1, compression2);
    }

    #[test]
    fn test_truncated_compression() {
        let compression1 = Compression::from_u8(1).unwrap();
        let compression2 = Compression::Truncated;

        assert_eq!(compression1, compression2);
    }

    #[test]
    fn test_rle_compression() {
        let compression1 = Compression::from_u8(4).unwrap();
        let compression2 = Compression::Rle;

        assert_eq!(compression1, compression2);
    }

    #[test]
    fn test_invalid_compression() {
        let compression = Compression::from_u8(2);

        assert_eq!(compression, Err(Error::InvalidCompression(2)));
    }
}
