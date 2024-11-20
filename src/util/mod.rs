pub fn is_twos_complement() -> bool {
    let mut is_twos_complement = false;
    let x = -1i32;
    let y = x as u32;
    if y == u32::MAX {
        is_twos_complement = true;
    }

    is_twos_complement
}

pub fn is_little_endian() -> bool {
    let mut is_little_endian = false;
    let x = 1u16;
    let y = x.to_le();
    if x == y {
        is_little_endian = true;
    }

    is_little_endian
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_twos_complement() {
        assert!(is_twos_complement());
    }

    #[test]
    fn test_is_little_endian() {
        assert!(is_little_endian());
    }
}
