use derive_builder::Builder;

use crate::sas::endianness::Endianness;
use crate::sas::page_binary::SasPageBinary;
use crate::sas::page_type::SasPageType;
use crate::sas::subheader_iterator::SasSubheaderIterator;

/// Represents the parsed data for a page in a sas7bdat file.
#[derive(Debug, Builder)]
pub struct SasPage<'a> {
    pub page_type: PageType,
    pub subheaders: Option<SasSubheaderIterator<'a>>,
    pub data_rows: Option<SasDataRowIterator<'a>>,
}

impl<'a> SasPage<'a> {
    pub fn from_binary_page(
        binary_page: &'a SasPageBinary,
        endianness: Endianness,
        alignment: &Alignment
    ) -> std::io::Result<SasPage<'a>> {
        // Prior code

        // Process subheaders
        let subheaders = binary_page.parse_subheaders(endianness, alignment)?;

        // Process data rows (if applicable)
        let data_rows = if binary_page.has_data_rows() {
            binary_page.parse_data_rows(endianness, alignment)?;
        } else {
            Vec::new()
        }

        Ok(
            SasPage {
                page_type: binary_page.page_type,
                subheaders,
                data_rows,
            }
        )
    }

    pub fn parse(
        binary_page: &'a SasPageBinary,
        is_little_endian: bool,
        alignment: Alignment,
    ) -> std::io::Result<Self> {
        let page_type = PageType::from_u16(binary_page.page_type);

        // Create subheader iterator if applicable
        let subheaders = if page_type.has_subheaders() {
            Some(SasSubheaderIterator::new(
                &binary_page.subheader_pointers,
                binary_page.subheader_count as usize,
                is_little_endian,
                alignment,
            ))
        } else {
            None
        };

        // Create data row iterator if applicable
        let data_rows = if page_type.has_data_rows() {
            let row_count = binary_page.get_row_count();
            let row_length = binary_page.get_row_length();
            Some(SasDataRowIterator::new(
                &binary_page.data,
                row_count,
                row_length,
            ))
        } else {
            None
        };

        Ok(SasPage {
            page_type,
            subheaders,
            data_rows,
        })
    }
}

// src/sas/parsed_page.rs

use super::binary_page::SasBinaryPage;
use super::subheader::{SasSubheader, SubheaderType};

pub struct SasPage {
    pub page_type: PageType,
    pub subheaders: Vec<SasSubheader>,
    pub data_rows: Vec<Vec<u8>>, // Each row as a vector of bytes
}

impl SasPage {
    pub fn parse(
        binary_page: &SasBinaryPage,
        is_little_endian: bool,
        alignment: &Alignment,
    ) -> io::Result<Self> {
        let page_type = PageType::from_u16(binary_page.page_type);

        // Parse subheader pointers
        let subheaders = SasSubheader::parse_subheader_pointers(
            &binary_page.subheader_pointers,
            binary_page.data.as_slice(),
            is_little_endian,
            alignment,
        )?;

        // Parse data rows if applicable
        let data_rows = match page_type {
            PageType::Data | PageType::Mix => {
                // Parse data rows
                // Implementation depends on the structure of data rows
                Vec::new() // Placeholder
            }
            _ => Vec::new(),
        };

        Ok(SasPage {
            page_type,
            subheaders,
            data_rows,
        })
    }
}

