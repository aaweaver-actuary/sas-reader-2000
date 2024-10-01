use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{self, Read, Seek};

/// Represents the raw binary for a page in a sas7bdat file.
pub struct SasBinaryPage {
    pub page_type: u16,
    pub block_count: u16,
    pub subheader_count: u16,
    pub subheader_pointers: Vec<u8>, // Raw bytes of subheader pointers
    pub data: Vec<u8>,               // Raw bytes of the page content
}

impl SasBinaryPage {
    pub fn read<R: Read + Seek>(reader: &mut R, page_size: usize) -> io::Result<Self> {
        let mut page_bytes = vec![0u8; page_size];
        reader.read_exact(&mut page_bytes)?;

        // Create a cursor for parsing the page header
        let mut cursor = std::io::Cursor::new(&page_bytes);

        // Parse the page header fields
        cursor.seek(std::io::SeekFrom::Start(16))?; // Offset to PGTYPE

        let page_type = cursor.read_u16::<LittleEndian>()?;
        let block_count = cursor.read_u16::<LittleEndian>()?;
        let subheader_count = cursor.read_u16::<LittleEndian>()?;

        // Calculate offsets and lengths
        let subheader_pointers_offset = 24;
        let subheader_pointers_length = subheader_count as usize * 12; // Assuming SL = 12 for 32-bit

        // Extract subheader pointers
        let subheader_pointers = page_bytes
            [subheader_pointers_offset..subheader_pointers_offset + subheader_pointers_length]
            .to_vec();

        // Extract data (rest of the page)
        let data_offset = subheader_pointers_offset + subheader_pointers_length;
        let data = page_bytes[data_offset..].to_vec();

        Ok(SasBinaryPage {
            page_type,
            block_count,
            subheader_count,
            subheader_pointers,
            data,
        })
    }
}
