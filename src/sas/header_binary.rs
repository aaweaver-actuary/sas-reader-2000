use std::io::{self, Read, Seek};

/// Represents the binary header of a SAS file.
#[derive(Debug)]
pub struct HeaderBinary {
    pub magic_number: [u8; 32],
    pub rest_of_header: Vec<u8>, // Holds the rest of the header bytes
}

impl HeaderBinary {
    pub fn read<R: Read + Seek>(reader: &mut R, header_length: usize) -> io::Result<Self> {
        let mut magic_number = [0u8; 32];
        reader.read_exact(&mut magic_number)?;

        let rest_length = header_length - 32;
        let mut rest_of_header = vec![0u8; rest_length];
        reader.read_exact(&mut rest_of_header)?;

        Ok(HeaderBinary {
            magic_number,
            rest_of_header,
        })
    }
}
