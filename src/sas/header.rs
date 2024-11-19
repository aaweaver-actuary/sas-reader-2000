const MAGIC_NUMBER: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea, 0x81, 0x60,
    0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c, 0x18, 0x1f, 0x10, 0x11,
];

#[derive(Debug)]
pub struct SasHeader {
    pub bytes: Vec<u8>,
}

impl SasHeader {
    pub fn new(bytes: &[u8]) -> Self {
        SasHeader {
            bytes: bytes.to_vec(),
        }
    }

    pub fn get_magic_number_from_header(&self) -> &[u8] {
        &self.bytes[0..32]
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

        if magic_number != MAGIC_NUMBER {
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
    use super::*;

    #[test]
    fn can_create_sas_header() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeader::new(bytes);

        assert_eq!(header.bytes, bytes);
    }

    #[test]
    fn can_validate_sas_file() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeader::new(bytes);

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
        let header = SasHeader::new(bytes);

        assert_eq!(header.get_magic_number_from_header(), &expected);
    }

    #[test]
    fn can_get_sas_file_from_header() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeader::new(bytes);

        assert_eq!(header.get_sas_file_string_from_header(), &bytes[84..92]);
    }

    #[test]
    fn can_get_sas_file_from_header_as_string() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeader::new(bytes);

        assert_eq!(
            header.get_sas_file_from_header_as_str(),
            "SAS FILE".to_string()
        );
    }

    #[test]
    fn can_get_sas_filename_from_header() {
        let bytes = include_bytes!("../../test/hadley.sas7bdat");
        let header = SasHeader::new(bytes);

        assert_eq!(
            header.get_sas_filename_from_header().to_lowercase(),
            "hadley".to_string()
        );
    }
}
