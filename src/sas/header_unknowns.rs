use std::collections::HashMap;

use crate::sas::Alignment;

pub type InnerMap = HashMap<String, usize>;
pub type UnknownsMap = HashMap<usize, InnerMap>;

fn make_inner_hash_map(start: usize, end: usize, value: usize) -> InnerMap {
    let mut map = HashMap::new();
    map.insert("range_start".to_string(), start);
    map.insert("range_end".to_string(), end);
    map.insert("value".to_string(), value);
    map
}

#[derive(Debug, PartialEq)]
pub struct SasHeaderUnknowns {
    pub bytes: Vec<u8>,
    pub unknowns: Option<UnknownsMap>,
}

impl SasHeaderUnknowns {
    pub fn new(bytes: &[u8]) -> Self {
        SasHeaderUnknowns {
            bytes: bytes.to_vec(),
            unknowns: None,
        }
    }

    fn insert_to_unknowns(&mut self, position: usize, range_start: usize, range_end: usize) {
        if self.unknowns.is_none() {
            self.unknowns = Some(HashMap::new());
        }
        self.unknowns.as_mut().unwrap().insert(
            position,
            make_inner_hash_map(range_start, range_end, self.bytes[position] as usize),
        );
    }

    fn insert_range(&mut self, range_start: usize, range_length: usize) {
        (range_start..range_start + range_length).for_each(|x| {
            self.insert_to_unknowns(x, range_start, range_start + range_length);
        });
    }

    pub fn read(&mut self) {
        let a1 = Alignment::from_u8(self.bytes[34]).value as usize;
        let a2 = Alignment::from_u8(self.bytes[32]).value as usize;

        let data = [
            (33, 2),
            (36, 1),
            (38, 1),
            (40, 8),
            (48, 8),
            (64, 6),
            (72, 12),
            (180 + a1, 16),
            (208 + a1 + a2, 8),
            (324 + a1 + a2, 4),
        ];

        data.iter().for_each(|(start, length)| {
            self.insert_range(*start, *length);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_inner_hash_map() {
        let map = make_inner_hash_map(1, 10, 5);
        assert_eq!(map.get("range_start"), Some(&1));
        assert_eq!(map.get("range_end"), Some(&10));
        assert_eq!(map.get("value"), Some(&5));
    }

    #[test]
    fn test_sas_header_unknowns_new() {
        let bytes = vec![0, 1, 2, 3, 4, 5];
        let header = SasHeaderUnknowns::new(&bytes);
        assert_eq!(header.bytes, bytes);
        assert!(header.unknowns.is_none());
    }

    #[test]
    fn test_insert_to_unknowns() {
        let bytes = vec![0, 1, 2, 3, 4, 5];
        let mut header = SasHeaderUnknowns::new(&bytes);
        header.insert_to_unknowns(1, 0, 2);
        let unknowns = header.unknowns.unwrap();
        let inner_map = unknowns.get(&1).unwrap();
        assert_eq!(inner_map.get("range_start"), Some(&0));
        assert_eq!(inner_map.get("range_end"), Some(&2));
        assert_eq!(inner_map.get("value"), Some(&(bytes[1] as usize)));
    }

    #[test]
    fn test_insert_range() {
        let bytes = vec![0, 1, 2, 3, 4, 5];
        let mut header = SasHeaderUnknowns::new(&bytes);
        header.insert_range(1, 3);
        let unknowns = header.unknowns.unwrap();
        (1..4).for_each(|i| {
            let inner_map = unknowns.get(&i).unwrap();
            assert_eq!(inner_map.get("range_start"), Some(&1));
            assert_eq!(inner_map.get("range_end"), Some(&4));
            assert_eq!(inner_map.get("value"), Some(&(bytes[i] as usize)));
        });
    }

    #[test]
    fn test_read() {
        let bytes = vec![0; 400];
        let mut header = SasHeaderUnknowns::new(&bytes);
        header.read();
        assert!(header.unknowns.is_some());
    }
}
