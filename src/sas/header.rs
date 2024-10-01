use crate::traits::ReadSeek;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{self, Read, Seek, SeekFrom};
use std::time::{Duration, UNIX_EPOCH};

#[derive(Debug)]
pub struct SasHeader {
    pub magic: [u8; 32],        // Offset 0, Length 32
    pub a2: u8,                 // Offset 32, Length 1
    pub mystery1: [u8; 2],      // Offset 33, Length 2 (Unknown)
    pub a1: u8,                 // Offset 35, Length 1
    pub mystery2: u8,           // Offset 36, Length 1 (Unknown)
    pub endian: u8,             // Offset 37, Length 1
    pub mystery3: u8,           // Offset 38, Length 1 (Unknown)
    pub os_type: u8,            // Offset 39, Length 1
    pub mystery4: [u8; 8],      // Offset 40, Length 8 (Unknown)
    pub mystery5: [u8; 8],      // Offset 48, Length 8 (Unknown)
    pub mystery6: [u8; 8],      // Offset 56, Length 8 (Repeat of bytes 32-40)
    pub mystery7: [u8; 6],      // Offset 64, Length 6 (Unknown)
    pub encoding: u16,          // Offset 70, Length 2
    pub mystery8: [u8; 12],     // Offset 72, Length 12 (Unknown)
    pub file_format: [u8; 8],   // Offset 84, Length 8 ('SAS FILE')
    pub dataset_name: [u8; 64], // Offset 92, Length 64
    pub file_type: [u8; 8],     // Offset 156, Length 8 (e.g., 'DATA   ')
    // Padding based on a1
    pub a1_padding: Vec<u8>, // Offset 164, Length a1 (Zero padding)
    // Timestamps
    pub date_created: f64,  // Offset 164 + a1, Length 8
    pub date_modified: f64, // Offset 172 + a1, Length 8
    pub mystery9: [u8; 16], // Offset 180 + a1, Length 16 (Unknown)
    pub header_length: u32, // Offset 196 + a1, Length 4
    pub page_size: u32,     // Offset 200 + a1, Length 4
    // Page count (length depends on u64 flag)
    pub page_count: u64, // Offset 204 + a1, Length 4 or 8
    // Further fields...
    // We will need to adjust offsets based on a1 and a2
    pub mystery10: [u8; 8],   // Offset 208 + a1 + a2, Length 8 (Unknown)
    pub sas_release: [u8; 8], // Offset 216 + a1 + a2, Length 8
    pub host: [u8; 16],       // Offset 224 + a1 + a2, Length 16
    pub os_version_number: [u8; 16], // Offset 240 + a1 + a2, Length 16
    pub os_maker: [u8; 16],   // Offset 256 + a1 + a2, Length 16
    pub os_name: [u8; 16],    // Offset 272 + a1 + a2, Length 16
                              // Remaining fields are unknown or low confidence
}

impl SasHeader {
    pub fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut header = SasHeader {
            magic: [0u8; 32],
            a2: 0,
            mystery1: [0u8; 2],
            a1: 0,
            mystery2: 0,
            endian: 0,
            mystery3: 0,
            os_type: 0,
            mystery4: [0u8; 8],
            mystery5: [0u8; 8],
            mystery6: [0u8; 8],
            mystery7: [0u8; 6],
            encoding: 0,
            mystery8: [0u8; 12],
            file_format: [0u8; 8],
            dataset_name: [0u8; 64],
            file_type: [0u8; 8],
            a1_padding: Vec::new(),
            date_created: 0.0,
            date_modified: 0.0,
            mystery9: [0u8; 16],
            header_length: 0,
            page_size: 0,
            page_count: 0,
            mystery10: [0u8; 8],
            sas_release: [0u8; 8],
            host: [0u8; 16],
            os_version_number: [0u8; 16],
            os_maker: [0u8; 16],
            os_name: [0u8; 16],
        };

        // Offset 0: Read magic number
        reader.read_exact(&mut header.magic)?;

        // Validate magic number
        let expected_magic_number: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea,
            0x81, 0x60, 0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c,
            0x18, 0x1f, 0x10, 0x11,
        ];

        if header.magic != expected_magic_number {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid SAS7BDAT magic number",
            ));
        }

        // Offset 32: Read a2
        header.a2 = reader.read_u8()?;

        // Determine u64 flag and a2 value
        let a2 = if header.a2 == 0x33 { 4 } else { 0 };
        let u64_flag = a2 == 4;

        // Offset 33: Read mystery1
        reader.read_exact(&mut header.mystery1)?;

        // Offset 35: Read a1
        header.a1 = reader.read_u8()?;

        // Determine a1 value
        let a1 = if header.a1 == 0x33 { 4 } else { 0 };

        // Offset 36: Read mystery2
        header.mystery2 = reader.read_u8()?;

        // Offset 37: Read endian
        header.endian = reader.read_u8()?;

        let little_endian = match header.endian {
            0x01 => true,  // Little endian
            0x00 => false, // Big endian
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown endianness: 0x{:02X}", header.endian),
                ))
            }
        };

        // Create appropriate byte order based on endianness
        let mut reader = if little_endian {
            Box::new(reader) as Box<dyn ReadSeek>
        } else {
            Box::new(reader) as Box<dyn ReadSeek>
        };

        // Offset 38: Read mystery3
        header.mystery3 = reader.read_u8()?;

        // Offset 39: Read OS type
        header.os_type = reader.read_u8()?;

        // Offset 40: Read mystery4
        reader.read_exact(&mut header.mystery4)?;

        // Offset 48: Read mystery5
        reader.read_exact(&mut header.mystery5)?;

        // Offset 56: Read mystery6
        reader.read_exact(&mut header.mystery6)?;

        // Offset 64: Read mystery7
        reader.read_exact(&mut header.mystery7)?;

        // Offset 70: Read encoding
        header.encoding = if little_endian {
            reader.read_u16::<LittleEndian>()?
        } else {
            reader.read_u16::<BigEndian>()?
        };

        // Offset 72: Read mystery8
        reader.read_exact(&mut header.mystery8)?;

        // Offset 84: Read file_format
        reader.read_exact(&mut header.file_format)?;

        // Offset 92: Read dataset_name
        reader.read_exact(&mut header.dataset_name)?;

        // Offset 156: Read file_type
        reader.read_exact(&mut header.file_type)?;

        // Offset 164: Read a1 padding
        if a1 > 0 {
            let mut padding = vec![0u8; a1 as usize];
            reader.read_exact(&mut padding)?;
            header.a1_padding = padding;
        }

        // Offsets after a1 padding
        // Read date_created and date_modified
        header.date_created = if little_endian {
            reader.read_f64::<LittleEndian>()?
        } else {
            reader.read_f64::<BigEndian>()?
        };

        header.date_modified = if little_endian {
            reader.read_f64::<LittleEndian>()?
        } else {
            reader.read_f64::<BigEndian>()?
        };

        // Read mystery9
        reader.read_exact(&mut header.mystery9)?;

        // Read header_length
        header.header_length = if little_endian {
            reader.read_u32::<LittleEndian>()?
        } else {
            reader.read_u32::<BigEndian>()?
        };

        // Read page_size
        header.page_size = if little_endian {
            reader.read_u32::<LittleEndian>()?
        } else {
            reader.read_u32::<BigEndian>()?
        };

        // Read page_count (length depends on u64 flag)
        if u64_flag {
            header.page_count = if little_endian {
                reader.read_u64::<LittleEndian>()?
            } else {
                reader.read_u64::<BigEndian>()?
            };
        } else {
            let page_count_32 = if little_endian {
                reader.read_u32::<LittleEndian>()?
            } else {
                reader.read_u32::<BigEndian>()?
            };
            header.page_count = page_count_32 as u64;
        }

        // Adjust offset for a2
        if a2 > 0 {
            // Skip a2 bytes (already read in page_count)
        }

        // Continue reading remaining fields
        // Read mystery10
        reader.read_exact(&mut header.mystery10)?;

        // Read sas_release
        reader.read_exact(&mut header.sas_release)?;

        // Read host
        reader.read_exact(&mut header.host)?;

        // Read os_version_number
        reader.read_exact(&mut header.os_version_number)?;

        // Read os_maker
        reader.read_exact(&mut header.os_maker)?;

        // Read os_name
        reader.read_exact(&mut header.os_name)?;

        // Return the populated header
        Ok(header)
    }
}
