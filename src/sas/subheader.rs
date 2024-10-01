use crate::sas::alignment::Alignment;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{self, Cursor, Read};

use crate::sas::endianness::Endianness;

pub trait SubheaderParser {
    fn parse(&self, data: &[u8], endianness: Endianness) -> std::io::Result<()>;
}

/// Represents the subheader of a page in a sas7bdat file.
#[derive(Debug)]
pub struct SasSubheader {
    pub offset: u64,
    pub length: u64,
    pub compression: u8,
    pub subheader_type: u8,
    pub data: Vec<u8>, // Raw subheader data
}

impl SasSubheader {
    pub fn parse_subheader_pointers(
        pointers_data: &[u8],
        page_data: &[u8],
        is_little_endian: bool,
        alignment: &Alignment,
    ) -> io::Result<Vec<Self>> {
        let mut subheaders = Vec::new();
        let mut cursor = Cursor::new(pointers_data);

        let pointer_size = if alignment.u64_flag { 24 } else { 12 };

        while (cursor.position() as usize) < pointers_data.len() {
            let offset = if alignment.u64_flag {
                if is_little_endian {
                    cursor.read_u64::<LittleEndian>()?
                } else {
                    cursor.read_u64::<BigEndian>()?
                }
            } else {
                let offset32 = if is_little_endian {
                    cursor.read_u32::<LittleEndian>()?
                } else {
                    cursor.read_u32::<BigEndian>()?
                };
                offset32 as u64
            };

            let length = if alignment.u64_flag {
                if is_little_endian {
                    cursor.read_u64::<LittleEndian>()?
                } else {
                    cursor.read_u64::<BigEndian>()?
                }
            } else {
                let length32 = if is_little_endian {
                    cursor.read_u32::<LittleEndian>()?
                } else {
                    cursor.read_u32::<BigEndian>()?
                };
                length32 as u64
            };

            let compression = cursor.read_u8()?;
            let subheader_type = cursor.read_u8()?;
            // Skip the remaining bytes in the pointer
            cursor.seek(std::io::SeekFrom::Current((pointer_size - 10) as i64))?;

            // If length is zero, we can ignore this subheader
            if length == 0 {
                continue;
            }

            // Extract subheader data
            let data_start = offset as usize;
            let data_end = data_start + length as usize;
            let data = page_data[data_start..data_end].to_vec();

            subheaders.push(SasSubheader {
                offset,
                length,
                compression,
                subheader_type,
                data,
            });
        }

        Ok(subheaders)
    }
}
