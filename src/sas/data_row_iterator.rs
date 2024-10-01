use std::io::{Cursor, Read, Result};

/// Represents an iterator over the data rows in a SAS data set.
///
/// The iterator reads rows of data from a buffer and returns them as byte vectors.
///
/// # Examples
/// ```
/// use sas_reader_2000::sas::SasDataRowIterator;
///
/// let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
/// let row_count = 2;
/// let row_length = 3;
/// let mut iterator = SasDataRowIterator::new(&data, row_count, row_length);
///
/// assert_eq!(iterator.next(), Some(Ok(vec![0x01, 0x02, 0x03])));
/// assert_eq!(iterator.next(), Some(Ok(vec![0x04, 0x05, 0x06])));
/// assert_eq!(iterator.next(), None);
/// ```
#[derive(Debug)]
pub struct SasDataRowIterator<'a> {
    cursor: Cursor<&'a [u8]>,
    remaining_rows: usize,
    row_length: usize,
}

impl<'a> SasDataRowIterator<'a> {
    pub fn new(data: &'a [u8], row_count: usize, row_length: usize) -> Self {
        SasDataRowIterator {
            cursor: Cursor::new(data),
            remaining_rows: row_count,
            row_length,
        }
    }
}

impl<'a> Iterator for SasDataRowIterator<'a> {
    type Item = Result<Vec<u8>>; // Or a more specific data row type

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_rows == 0 {
            return None;
        }
        self.remaining_rows -= 1;

        let mut row_data = vec![0u8; self.row_length];
        if let Err(e) = self.cursor.read_exact(&mut row_data) {
            return Some(Err(e));
        }

        Some(Ok(row_data))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    /// Test that the iterator reads the data correctly.
    /// Expected: The iterator should return two rows of data, each with 3 bytes.
    /// The first row should be [0x01, 0x02, 0x03] and the second row should be
    /// [0x04, 0x05, 0x06]. When the iterator reaches the end of the data, it
    /// should return `None`.
    #[test]
    fn test_sas_data_row_iterator() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let row_count = 2;
        let row_length = 3;
        let mut iterator = SasDataRowIterator::new(&data, row_count, row_length);

        let next: Vec<u8> = iterator
            .next()
            .expect("Expected Ok(_) value, found Err(_) instead")
            .expect("Expected Ok(_) value, found Err(_) instead");
        assert_eq!(next, vec![0x01, 0x02, 0x03]);

        let next: Vec<u8> = iterator
            .next()
            .expect("Expected Ok(_) value, found Err(_) instead")
            .expect("Expected Ok(_) value, found Err(_) instead");
        assert_eq!(next, vec![0x04, 0x05, 0x06]);

        let next: Option<Result<Vec<u8>>> = iterator.next();
        assert!(next.is_none());
    }

    /// Test that the iterator returns an error when the buffer is incomplete.
    /// Expected: Since `data` has 5 bytes and `row_length` is 3, the iterator
    /// should return an error when trying to read the second row, because it
    /// expects 3 bytes but only 2 are available.
    #[test]
    fn test_sas_data_row_iterator_incomplete_row() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let row_count = 2;
        let row_length = 3;
        let mut iterator = SasDataRowIterator::new(&data, row_count, row_length);

        // First row should be read successfully
        assert_eq!(iterator.next().unwrap().unwrap(), vec![0x01, 0x02, 0x03]);

        // Second row should return an error
        assert_eq!(
            iterator.next().unwrap().unwrap_err().kind(),
            ErrorKind::UnexpectedEof
        );

        // No more rows should be available
        assert!(iterator.next().is_none());
    }
}
