#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCompression(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let error = Error::InvalidCompression(0);
        assert_eq!(error, Error::InvalidCompression(0));
    }
}
