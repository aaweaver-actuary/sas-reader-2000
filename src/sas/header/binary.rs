use crate::sas::{Encoding, Endianness, FileType, OsType, SasConstants};

#[derive(Debug, PartialEq)]
pub struct SasHeaderBinary {
    pub bytes: Vec<u8>,
}

impl SasHeaderBinary {
    pub fn new(bytes: &[u8]) -> Self {
        SasHeaderBinary {
            bytes: bytes.to_vec(),
        }
    }

    pub fn get_magic_number_from_header(&self) -> &[u8] {
        &self.bytes[0..32]
    }

    pub fn get_alignment_from_header1(&self) -> u8 {
        if self.bytes[32] == 0x33 {
            4
        } else {
            0
        }
    }

    pub fn get_alignment_from_header2(&self) -> u8 {
        if self.bytes[35] == 0x33 {
            4
        } else {
            0
        }
    }

    pub fn get_a1(&self) -> u8 {
        self.get_alignment_from_header2()
    }

    pub fn get_a2(&self) -> u8 {
        self.get_alignment_from_header1()
    }

    pub fn get_endianness_from_header(&self) -> Result<Endianness, String> {
        Endianness::from_u8(self.bytes[37])
    }

    pub fn get_os_type_from_header(&self) -> OsType {
        OsType::from_u8(self.bytes[39])
    }

    pub fn get_character_encoding_from_header(&self) -> Result<Encoding, String> {
        let encoding_byte = self.bytes[70];
        Encoding::from_u8(encoding_byte)
    }

    pub fn get_sas_file_string_from_header(&self) -> &[u8] {
        &self.bytes[84..92]
    }

    pub fn get_sas_file_from_header_as_str(&self) -> String {
        String::from_utf8_lossy(self.get_sas_file_string_from_header())
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn get_sas_filename_from_header(&self) -> String {
        String::from_utf8_lossy(&self.bytes[92..156])
            .trim_end_matches('\0')
            .trim_end()
            .to_string()
    }

    pub fn is_56_to_64_valid(&self) -> bool {
        let positions_32_to_40 = &self.bytes[32..40];
        let positions_56_to_64 = &self.bytes[56..64];

        (0..8).all(|i| positions_32_to_40[i] == positions_56_to_64[i])
    }

    pub fn validate_sas_file(&self) -> Result<(), String> {
        let magic_number = self.get_magic_number_from_header();
        let sas_file = self.get_sas_file_from_header_as_str();
        let sas_filename = self.get_sas_filename_from_header();
        let constants = SasConstants::new();

        if magic_number != constants.magic_number {
            return Err("Invalid magic number".to_string());
        }

        if sas_file != "SAS FILE" {
            return Err("Invalid SAS file".to_string());
        }

        if sas_filename.is_empty() {
            return Err("Invalid SAS filename".to_string());
        }

        Ok(())
    }

    pub fn get_raw_ascii_file_type_from_header(&self) -> Result<String, String> {
        let positions_156_to_164 = &self.bytes[156..164];
        let ascii_file_type = String::from_utf8_lossy(positions_156_to_164)
            .trim_end_matches('\0')
            .to_string();

        if ascii_file_type.is_empty() {
            return Err("Invalid ASCII file type".to_string());
        }

        Ok(ascii_file_type)
    }

    pub fn get_ascii_file_type_from_header(&self) -> Result<FileType, String> {
        let raw_file_type = self.get_raw_ascii_file_type_from_header()?;

        FileType::from_str(&raw_file_type)
    }

    pub fn get_ts_offset(&self) -> usize {
        let a1 = self.get_alignment_from_header2();
        164 + a1 as usize
    }

    pub fn get_creation_timestamp(&self) -> u64 {
        let ts_offset = self.get_ts_offset();
        let timestamp_bytes = &self.bytes[ts_offset..ts_offset + 8];
        u64::from_le_bytes(timestamp_bytes.try_into().unwrap())
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn can_create_sas_header() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(header.bytes, bytes);
    }

    #[test]
    fn can_validate_sas_file() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert!(header.validate_sas_file().is_ok());
    }

    #[test]
    fn can_get_magic_number_from_header() {
        let expected: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea,
            0x81, 0x60, 0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c,
            0x18, 0x1f, 0x10, 0x11,
        ];

        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(header.get_magic_number_from_header(), &expected);
    }

    #[test]
    fn can_get_sas_file_from_header() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(header.get_sas_file_string_from_header(), &bytes[84..92]);
    }

    #[test]
    fn can_get_sas_file_from_header_as_string() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(
            header.get_sas_file_from_header_as_str(),
            "SAS FILE".to_string()
        );
    }

    #[test]
    fn can_get_sas_filename_from_header() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(
            header.get_sas_filename_from_header().to_lowercase(),
            "hadley".to_string()
        );
    }

    #[test]
    fn can_get_alignment_from_header1() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);
        let expected = if bytes[32] == 0x33 { 4 } else { 0 };

        assert_eq!(header.get_alignment_from_header1(), expected);
    }

    #[test]
    fn can_get_alignment_from_header2() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);
        let expected = if bytes[35] == 0x33 { 4 } else { 0 };

        assert_eq!(header.get_alignment_from_header2(), expected);
    }

    #[test]
    fn can_get_endianness_from_header() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);
        let endianness_from_bytes = Endianness::from_u8(bytes[37]);
        let endianness_from_header = header.get_endianness_from_header();

        assert_eq!(endianness_from_bytes, endianness_from_header);
    }

    #[test]
    fn can_get_os_type_from_header_when_unix() {
        let mut bytes = vec![0_u8; 8192];
        bytes[39] = 1;

        let header = SasHeaderBinary::new(bytes.as_slice());

        // 1 = Unix, 2 = Windows
        assert_eq!(header.get_os_type_from_header(), OsType::Unix);
    }

    #[test]
    fn can_get_os_type_from_header_when_windows() {
        let mut bytes = vec![0_u8; 8192];
        bytes[39] = 2;

        let header = SasHeaderBinary::new(bytes.as_slice());

        // 1 = Unix, 2 = Windows
        assert_eq!(header.get_os_type_from_header(), OsType::Windows);
    }

    #[test]
    #[should_panic(
        expected = "Unknown OS type code from binary: 3. Expected either 1 for Unix or 2 for Windows."
    )]
    fn cannot_get_os_type_from_header_when_invalid() {
        let mut bytes = vec![0_u8; 8192];
        bytes[39] = 3; // Invalid OS type -> should panic

        let header = SasHeaderBinary::new(bytes.as_slice());

        // 1 = Unix, 2 = Windows
        header.get_os_type_from_header();
    }

    #[test]
    fn can_validate_positions_56_to_64_when_actually_valid() {
        // Validate that positions 56 to 64 match positions 32 to 40
        let valid_bytes = (0..8192)
            .map(|i| {
                if (32..=40).contains(&i) {
                    (i - 32 + 1) as u8
                } else if (56..=63).contains(&i) {
                    (i - 56 + 1) as u8
                } else {
                    0
                }
            })
            .collect::<Vec<u8>>();

        let header = SasHeaderBinary::new(&valid_bytes);
        assert!(header.is_56_to_64_valid());
    }

    #[test]
    fn can_validate_positions_56_to_64_when_actually_invalid() {
        let invalid_bytes = (0..8192)
            .map(|i| {
                if (56..=63).contains(&i) {
                    (i - 56 + 2) as u8
                } else {
                    0
                }
            })
            .collect::<Vec<u8>>();

        let header = SasHeaderBinary::new(&invalid_bytes);
        assert!(!header.is_56_to_64_valid());
    }

    #[test]
    fn can_get_character_encoding() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        let encoding = header.get_character_encoding_from_header();
        assert_eq!(encoding, Ok(Encoding::Windows1252));
    }

    #[test]
    fn can_get_character_encoding_when_latin1() {
        let mut bytes = vec![0_u8; 8192];
        bytes[70] = 29;

        let header = SasHeaderBinary::new(bytes.as_slice());

        let encoding = header.get_character_encoding_from_header();
        assert_eq!(encoding, Ok(Encoding::Iso8859_1));
    }

    #[test]
    fn can_get_ascii_file_type_from_header() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        let raw_file_type = header.get_raw_ascii_file_type_from_header();
        assert_eq!(raw_file_type, Ok("DATA    ".to_string()));

        let file_type = header.get_ascii_file_type_from_header();
        assert_eq!(file_type, Ok(FileType::Data));
    }

    #[test]
    fn test_get_a1_when_it_should_be_0() {
        let bytes = vec![0_u8; 8192];

        let header = SasHeaderBinary::new(&bytes);
        let a1 = header.get_a1();
        assert_eq!(a1, 0);
    }

    #[test]
    fn test_get_a1_when_it_should_be_4() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        let header = SasHeaderBinary::new(&bytes);
        let a1 = header.get_a1();
        assert_eq!(a1, 4);
    }

    #[test]
    fn test_get_a2_when_it_should_be_0() {
        let bytes = vec![0_u8; 8192];

        let header = SasHeaderBinary::new(&bytes);
        let a2 = header.get_a2();
        assert_eq!(a2, 0);
    }

    #[test]
    fn test_get_a2_when_it_should_be_4() {
        let mut bytes = vec![0_u8; 8192];
        bytes[32] = 0x33; // This makes a2=4

        let header = SasHeaderBinary::new(&bytes);
        let a2 = header.get_a2();
        assert_eq!(a2, 4);
    }

    #[test]
    fn can_get_creation_timestamp_offset_when_a1_eq_0() {
        let bytes = vec![0_u8; 8192];

        let header = SasHeaderBinary::new(&bytes);
        let a1 = header.get_a1();
        assert_eq!(a1, 0);

        let ts_offset = header.get_ts_offset();
        assert_eq!(ts_offset, 164 + a1 as usize);
    }

    #[test]
    fn can_get_creation_timestamp_offset_when_a1_eq_4() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        let header = SasHeaderBinary::new(&bytes);
        let a1 = header.get_a1();
        assert_eq!(a1, 4);

        let ts_offset = header.get_ts_offset();
        assert_eq!(ts_offset, 164 + a1 as usize);
    }

    #[test]
    fn can_get_creation_timestamp() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        let ts = header.get_creation_timestamp();
        assert_eq!(ts, 4745081191338971496);
    }

    #[test]
    fn can_get_inserted_ts_from_midnight_11_26_1987() {
        let mut bytes = vec![0_u8; 8192];
        bytes[164] = 0x00;
        bytes[165] = 0x00;
        bytes[166] = 0x00;
        bytes[167] = 0x00;
        bytes[168] = 0x00;
        bytes[169] = 0x00;
        bytes[170] = 0x00;
        bytes[171] = 0x00;

        let header = SasHeaderBinary::new(&bytes);
        let ts = header.get_creation_timestamp();
        assert_eq!(ts, 0);
    }
}
