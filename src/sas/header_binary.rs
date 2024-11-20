use crate::sas::{Endianness, OsType, SasConstants};

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
        if self.bytes[34] == 0x33 {
            4
        } else {
            0
        }
    }

    pub fn get_endianness_from_header(&self) -> Endianness {
        Endianness::from_u8(self.bytes[36])
    }

    pub fn get_os_type_from_header(&self) -> OsType {
        OsType::from_u8(self.bytes[39])
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
}

#[cfg(test)]

mod tests {
    use crate::sas::Endianness;

    use super::*;

    #[test]
    fn can_create_sas_header() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(header.bytes, bytes);
    }

    #[test]
    fn can_validate_sas_file() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
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

        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(header.get_magic_number_from_header(), &expected);
    }

    #[test]
    fn can_get_sas_file_from_header() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(header.get_sas_file_string_from_header(), &bytes[84..92]);
    }

    #[test]
    fn can_get_sas_file_from_header_as_string() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(
            header.get_sas_file_from_header_as_str(),
            "SAS FILE".to_string()
        );
    }

    #[test]
    fn can_get_sas_filename_from_header() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(
            header.get_sas_filename_from_header().to_lowercase(),
            "hadley".to_string()
        );
    }

    #[test]
    fn can_get_alignment_from_header1() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);
        let expected = if bytes[32] == 0x33 { 4 } else { 0 };

        assert_eq!(header.get_alignment_from_header1(), expected);
    }

    #[test]
    fn can_get_alignment_from_header2() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);
        let expected = if bytes[34] == 0x33 { 4 } else { 0 };

        assert_eq!(header.get_alignment_from_header2(), expected);
    }

    #[test]
    fn can_get_endianness_from_header() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);
        let endianness_from_bytes = Endianness::from_u8(bytes[36]);
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
}
