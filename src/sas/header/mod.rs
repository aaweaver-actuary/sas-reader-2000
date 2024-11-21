pub mod alignment;
pub mod binary;
pub mod encoding;
pub mod endianness;
pub mod unknowns;
pub mod os_type;
pub mod file_type;

pub use alignment::Alignment;
pub use binary::SasHeaderBinary;
pub use os_type::OsType;
pub use encoding::Encoding;
pub use endianness::Endianness;
pub use unknowns::SasHeaderUnknowns;
pub use file_type::FileType;

#[derive(Debug, PartialEq)]
pub struct SasHeader {
    pub binary: SasHeaderBinary,
    pub magic_number: Option<[u8; 32]>,
    pub unknowns: Option<SasHeaderUnknowns>,
    pub alignment: Option<(Alignment, Alignment)>,
    pub endianness: Option<Endianness>,
    pub encoding: Option<Encoding>,
    pub sas_filename: Option<String>,
}

impl SasHeader {
    pub fn new(bytes: &[u8]) -> Self {
        SasHeader {
            binary: SasHeaderBinary::new(bytes),
            magic_number: None,
            unknowns: None,
            alignment: None,
            endianness: None,
            encoding: None,
            sas_filename: None,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.binary.bytes
    }

    pub fn read_magic_number(&mut self) {
        let magic_number = self.binary.get_magic_number_from_header();
        self.magic_number = Some(magic_number.try_into().unwrap());
    }

    pub fn read_alignment(&mut self) {
        // Note that alignment1 is a1 and alignment2 is a2
        // and also a1 comes from the 2nd alignment byte in the header
        // while a2 comes from the 1st alignment byte in the header
        let alignment1: Alignment = Alignment {
            value: self.binary.get_alignment_from_header2(),
        };
        let alignment2: Alignment = Alignment {
            value: self.binary.get_alignment_from_header1(),
        };

        self.alignment = Some((alignment1, alignment2));
    }

    pub fn is_u64_file_format(&mut self) -> bool {
        if self.alignment.is_none() {
            self.read_alignment();
        }

        let alignment = self.alignment.unwrap();
        alignment.1.value == 4
    }

    pub fn byte_controling_offset_before_timestamps(&mut self) -> u8 {
        if self.alignment.is_none() {
            self.read_alignment();
        }

        let alignment = self.alignment.unwrap();
        alignment.0.value
    }

    pub fn read_endianness(&mut self) {
        let endianness = self.binary.get_endianness_from_header();
        match endianness {
            Ok(endianness) => self.endianness = Some(endianness),
            Err(e) => panic!("{}", e),
        }
    }

    pub fn read_unknowns(&mut self) {
        let mut header_unknowns = SasHeaderUnknowns::new(self.bytes());
        header_unknowns.read();

        self.unknowns = Some(header_unknowns);
    }

    pub fn read_character_encoding(&mut self) {
        let encoding = self.binary.get_character_encoding_from_header();
        match encoding {
            Ok(encoding) => self.encoding = Some(encoding),
            Err(e) => panic!("{}", e),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::sas::SasConstants;

    #[test]
    fn can_create_sas_header() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let sas_header1 = SasHeader::new(bytes);
        let sas_header2 = SasHeader {
            binary: SasHeaderBinary::new(bytes),
            magic_number: None,
            unknowns: None,
            alignment: None,
            endianness: None,
            encoding: None,
            sas_filename: None,
        };

        assert_eq!(sas_header1, sas_header2);
    }

    #[test]
    fn can_return_bytes() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let sas_header = SasHeader::new(bytes);

        assert_eq!(sas_header.bytes(), bytes);
    }

    #[test]
    fn can_read_unknowns() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let mut sas_header = SasHeader::new(bytes);
        sas_header.read_unknowns();

        let header_unknowns = sas_header.unknowns.unwrap();
        let mut struct_unknowns = SasHeaderUnknowns::new(bytes);

        struct_unknowns.read();

        assert_eq!(header_unknowns.bytes, bytes);
        assert_eq!(header_unknowns, struct_unknowns);
    }

    #[test]
    fn can_read_magic_number() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let mut sas_header = SasHeader::new(bytes);
        sas_header.read_magic_number();

        let magic_number = sas_header.magic_number.unwrap();
        let constants = SasConstants::new();

        assert_eq!(magic_number, constants.magic_number);
    }

    #[test]
    fn can_read_alignment1() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let mut sas_header = SasHeader::new(bytes);
        sas_header.read_alignment();

        let alignment = sas_header.alignment.unwrap();

        assert!(alignment.0 == Alignment::from_u8(0) || alignment.0 == Alignment::from_u8(0x33));
        assert!(alignment.0.value == 0 || alignment.0.value == 4);
    }

    #[test]
    fn can_read_alignment2() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let mut sas_header = SasHeader::new(bytes);
        sas_header.read_alignment();

        let alignment = sas_header.alignment.unwrap();

        assert!(alignment.1 == Alignment::from_u8(0) || alignment.1 == Alignment::from_u8(0x33));
        assert!(alignment.1.value == 0 || alignment.1.value == 4);
    }

    #[test]
    fn can_read_endianess() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let mut sas_header = SasHeader::new(bytes);
        sas_header.read_endianness();

        let endianness = sas_header.endianness;

        assert!(
            endianness.unwrap() == Endianness::Big || endianness.unwrap() == Endianness::Little
        );
    }

    #[test]
    fn test_is_u64_file_format() {
        let mut bytes = vec![0_u8; 8192];
        println!("bytes[32] before change: {:#x}", bytes[32]);
        bytes[32] = 0x33; // This makes a2=4, so it's a u64 file format
        println!("bytes[32] after change: {:#x}", bytes[32]);

        let mut sas_header = SasHeader::new(bytes.as_slice());
        sas_header.read_alignment();
        println!("{:#?}", sas_header.alignment);

        assert!(sas_header.is_u64_file_format());

        let mut sas_header_without_explicitly_reading_alignment = SasHeader::new(bytes.as_slice());
        assert!(sas_header_without_explicitly_reading_alignment.is_u64_file_format());

        bytes[32] = 0x00; // This makes a2=0, so it's not a u64 file format
        let mut non64_sas_header = SasHeader::new(bytes.as_slice());

        assert!(!non64_sas_header.is_u64_file_format());
    }

    #[test]
    fn test_byte_controling_offset_before_timestamps1() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        let mut sas_header = SasHeader::new(bytes.as_slice());
        sas_header.read_alignment();

        assert_eq!(sas_header.byte_controling_offset_before_timestamps(), 4);
    }

    #[test]
    fn test_byte_controling_offset_before_timestamps2() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        let mut sas_header_without_explicitly_reading_alignment = SasHeader::new(bytes.as_slice());
        assert_eq!(
            sas_header_without_explicitly_reading_alignment
                .byte_controling_offset_before_timestamps(),
            4
        );
    }

    #[test]
    fn test_byte_controling_offset_before_timestamps3() {
        let bytes = vec![0_u8; 8192];
        // bytes[35] = 0x00; // Do not change a1, so it's 0

        let mut sas_header_with_no_timestamp_offset = SasHeader::new(bytes.as_slice());
        assert_eq!(
            sas_header_with_no_timestamp_offset.byte_controling_offset_before_timestamps(),
            0
        );
    }

    #[test]
    fn can_get_character_encoding() {
        let mut bytes = vec![0_u8; 8192];
        bytes[70] = 20; // This makes the character encoding to be UTF-8

        let mut sas_header = SasHeader::new(bytes.as_slice());
        sas_header.read_character_encoding();

        assert_eq!(sas_header.encoding, Some(Encoding::Utf8));
    }
}
