use crate::sas::page_binary::SasBinaryPage;
use crate::traits::ReadSeek;
use std::io::{Error, Result};

/// Represents the iterator for reading pages in a sas7bdat file.
#[derive(Debug)]
pub struct SasPageIterator<'a, R: ReadSeek> {
    reader: &'a mut R,
    page_size: usize,
    remaining_pages: usize,
}

impl<'a, R: ReadSeek> SasPageIterator<'a, R> {
    /// Creates a new `SasPageIterator`.
    ///
    /// # Arguments
    ///
    /// * `reader` - The reader to use for reading the pages.
    /// * `page_size` - The size of each page in the file.
    /// * `page_count` - The number of pages in the file.
    ///
    /// # Returns
    ///
    /// A new `SasPageIterator`.
    fn new(reader: &'a mut R, page_size: usize, page_count: usize) -> Self {
        SasPageIterator {
            reader,
            page_size,
            remaining_pages: page_count,
        }
    }
}

impl<'a, R: ReadSeek> Iterator for SasPageIterator<'a, R> {
    type Item = Result<SasBinaryPage, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_pages == 0 {
            return None;
        }
        self.remaining_pages -= 1;
        Some(SasBinaryPage::parse(self.reader))
    }
}
