use crate::sas::{Encoding, Endianness, FileType, OsMaker, OsType, SasConstants};

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

    pub fn get_os_type_from_header(&self) -> Result<OsType, String> {
        let char_value = self.bytes[39] as char;
        OsType::from_u8(char_value as u8)
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

        if !self.is_56_to_64_valid() {
            return Err("Invalid positions 56 to 64".to_string());
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

    fn get_creation_ts_offset(&self) -> usize {
        164 + self.get_a1() as usize
    }

    pub fn get_creation_timestamp_from_header(&self) -> f64 {
        let ts_offset = self.get_creation_ts_offset();
        let timestamp_bytes = &self.bytes[ts_offset..ts_offset + 8];
        f64::from_le_bytes(timestamp_bytes.try_into().unwrap())
    }

    fn get_modification_ts_offset(&self) -> usize {
        172 + self.get_a1() as usize
    }

    pub fn get_modification_timestamp_from_header(&self) -> f64 {
        let ts_offset = self.get_modification_ts_offset();
        let timestamp_bytes = &self.bytes[ts_offset..ts_offset + 8];
        f64::from_le_bytes(timestamp_bytes.try_into().unwrap())
    }

    fn get_header_len_offset(&self) -> usize {
        196 + self.get_a1() as usize
    }

    pub fn get_header_length_from_header(&self) -> usize {
        let header_len_offset = self.get_header_len_offset();
        let bytes = &self.bytes[header_len_offset..(header_len_offset + 4)];

        i32::from_le_bytes(bytes.try_into().unwrap()) as usize
    }

    fn get_page_size_len_offset(&self) -> usize {
        200 + self.get_a1() as usize
    }

    fn get_page_size_len_slice_range(&self) -> std::ops::Range<usize> {
        let page_size_offset = self.get_page_size_len_offset();
        let page_size_max_offset = page_size_offset + 4;
        page_size_offset..page_size_max_offset
    }

    pub fn get_page_size_from_header(&self) -> usize {
        let page_size_slice_range = self.get_page_size_len_slice_range();
        let bytes = &self.bytes[page_size_slice_range];

        i32::from_le_bytes(bytes.try_into().unwrap()) as usize
    }

    fn get_page_count_len_min_offset(&self) -> usize {
        204 + self.get_a1() as usize
    }

    fn get_page_count_len(&self) -> usize {
        match self.get_a2() {
            0 => 4,
            4 => 8,
            _ => panic!("Invalid a2 value: {}", self.get_a2()),
        }
    }

    /// Get the page count from the header.
    /// The page count is stored as either a 4-byte or 8-byte integer.
    /// The length of the integer is determined by the value of `a2`.
    /// If `a2` is 0, the page count is stored as a 4-byte integer.
    /// If `a2` is 4, the page count is stored as an 8-byte integer.
    /// The page count is stored at offset 204 + `a1`.
    /// The integer ranges from 204 + `a1` to 204 + `a1` + `a2`.
    pub fn get_page_count_from_header(&self) -> usize {
        let pc_min_offset = self.get_page_count_len_min_offset();
        let pc_len = self.get_page_count_len();
        let bytes = &self.bytes[pc_min_offset..pc_min_offset + pc_len];

        match pc_len {
            4 => i32::from_le_bytes(bytes.try_into().unwrap()) as usize,
            8 => i64::from_le_bytes(bytes.try_into().unwrap()) as usize,
            _ => panic!(
                "Invalid page count length: got {}, expected 4 or 8.\nself.get_a2(): {}",
                pc_len,
                self.get_a2()
            ),
        }
    }

    pub fn get_sas_release_from_header(&self) -> String {
        let a1 = self.get_a1() as usize;
        let a2 = self.get_a2() as usize;
        let sas_release_offset = 216 + a1 + a2;

        let sas_release_bytes = &self.bytes[sas_release_offset..sas_release_offset + 8];
        String::from_utf8_lossy(sas_release_bytes)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn get_host_sas_server_type_from_header(&self) -> String {
        let a1 = self.get_a1() as usize;
        let a2 = self.get_a2() as usize;
        let host_sas_server_type_offset = 224 + a1 + a2;

        let host_sas_server_type_bytes =
            &self.bytes[host_sas_server_type_offset..host_sas_server_type_offset + 16];
        String::from_utf8_lossy(host_sas_server_type_bytes)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn get_os_version_number_from_header(&self) -> Option<String> {
        let a1 = self.get_a1() as usize;
        let a2 = self.get_a2() as usize;
        let os_version_number_start = 240 + a1 + a2;
        let os_version_number_len = 16;
        let os_version_number_end = os_version_number_start + os_version_number_len;

        let os_version_number_bytes = &self.bytes[os_version_number_start..os_version_number_end];
        let os_version_number = String::from_utf8_lossy(os_version_number_bytes)
            .trim_end_matches('\0')
            .to_string();

        if self.get_os_type_from_header().unwrap() != OsType::Unix {
            None
        } else {
            Some(os_version_number)
        }
    }

    pub fn get_os_maker_or_version_from_header(&self) -> Option<OsMaker> {
        let a1 = self.get_a1() as usize;
        let a2 = self.get_a2() as usize;
        let os_version_start = 256 + a1 + a2;
        let os_version_len = 16;
        let os_version_end = os_version_start + os_version_len;

        let os_version_bytes = &self.bytes[os_version_start..os_version_end];
        let os_version_str = String::from_utf8_lossy(os_version_bytes)
            .trim_end_matches('\0')
            .to_string();

        let os_version = OsMaker::from_ascii(os_version_str);

        if self.get_os_type_from_header().unwrap() != OsType::Unix {
            None
        } else {
            match os_version {
                Ok(os_version) => Some(os_version),
                Err(_) => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::time::get_sas_epoch;
    use assert_approx_eq::assert_approx_eq;
    use chrono::NaiveDateTime;

    use super::*;

    fn fix_bytes_for_a1_4(bytes: &mut [u8]) {
        bytes[35] = 0x33; // This makes a1=4
    }

    fn fix_bytes_for_a2_4(bytes: &mut [u8]) {
        bytes[32] = 0x33; // This makes a2=4
    }

    fn header_from_test_file() -> SasHeaderBinary {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        SasHeaderBinary::new(bytes)
    }

    #[test]
    fn can_create_sas_header() {
        let bytes = include_bytes!("../../../test/hadley.sas7bdat");
        let header = SasHeaderBinary::new(bytes);

        assert_eq!(header.bytes, bytes);
    }

    #[test]
    fn can_validate_sas_file() {
        let header = header_from_test_file();
        assert!(header.validate_sas_file().is_ok());
    }

    #[test]
    fn can_get_magic_number_from_header() {
        let expected: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea,
            0x81, 0x60, 0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c,
            0x18, 0x1f, 0x10, 0x11,
        ];

        let header = header_from_test_file();
        assert_eq!(header.get_magic_number_from_header(), &expected);
    }

    #[test]
    fn can_get_sas_file_from_header() {
        let header = header_from_test_file();
        let bytes = &header.bytes;
        assert_eq!(header.get_sas_file_string_from_header(), &bytes[84..92]);
    }

    #[test]
    fn can_get_sas_file_from_header_as_string() {
        let header = header_from_test_file();
        assert_eq!(
            header.get_sas_file_from_header_as_str(),
            "SAS FILE".to_string()
        );
    }

    #[test]
    fn can_get_sas_filename_from_header() {
        let header = header_from_test_file();
        assert_eq!(
            header.get_sas_filename_from_header().to_lowercase(),
            "hadley".to_string()
        );
    }

    #[test]
    fn can_get_alignment_from_header1() {
        let header = header_from_test_file();
        let bytes = &header.bytes;
        let expected = if bytes[32] == 0x33 { 4 } else { 0 };

        assert_eq!(header.get_alignment_from_header1(), expected);
    }

    #[test]
    fn can_get_alignment_from_header2() {
        let header = header_from_test_file();
        let bytes = &header.bytes;
        let expected = if bytes[35] == 0x33 { 4 } else { 0 };

        assert_eq!(header.get_alignment_from_header2(), expected);
    }

    #[test]
    fn can_get_endianness_from_header() {
        let header = header_from_test_file();
        let bytes = &header.bytes;
        let endianness_from_bytes = Endianness::from_u8(bytes[37]);
        let endianness_from_header = header.get_endianness_from_header();

        assert_eq!(endianness_from_bytes, endianness_from_header);
    }

    #[test]
    fn can_get_os_type_from_header_when_unix() {
        let mut bytes = vec![0_u8; 8192];
        bytes[39] = b'1';

        let header = SasHeaderBinary::new(bytes.as_slice());

        // 1 = Unix, 2 = Windows
        assert_eq!(header.get_os_type_from_header().unwrap(), OsType::Unix);
    }

    #[test]
    fn can_get_os_type_from_header_when_windows() {
        let mut bytes = vec![0_u8; 8192];
        bytes[39] = b'2';

        let header = SasHeaderBinary::new(bytes.as_slice());

        // 1 = Unix, 2 = Windows
        assert_eq!(header.get_os_type_from_header().unwrap(), OsType::Windows);
    }

    #[test]
    #[should_panic(
        expected = "Unknown OS type code from binary: 3. Expected either 1 for Unix or 2 for Windows."
    )]
    fn cannot_get_os_type_from_header_when_invalid() {
        let mut bytes = vec![0_u8; 8192];
        bytes[39] = b'3'; // Invalid OS type -> should panic

        let header = SasHeaderBinary::new(bytes.as_slice());

        // 1 = Unix, 2 = Windows
        header.get_os_type_from_header().unwrap();
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
        let header = header_from_test_file();
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
        let header = header_from_test_file();
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
        fix_bytes_for_a1_4(&mut bytes);

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

        let ts_offset = header.get_creation_ts_offset();
        assert_eq!(ts_offset, 164 + a1 as usize);
    }

    #[test]
    fn can_get_creation_timestamp_offset_when_a1_eq_4() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        let header = SasHeaderBinary::new(&bytes);
        let a1 = header.get_a1();
        assert_eq!(a1, 4);

        let ts_offset = header.get_creation_ts_offset();
        assert_eq!(ts_offset, 164 + a1 as usize);
    }

    #[test]
    fn can_get_inserted_ts_from_midnight_11_26_1987() {
        let mut bytes = vec![0_u8; 8192];

        // 1987-11-26 00:00:00
        let end_ts = NaiveDateTime::parse_from_str("1987-11-26 00:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .timestamp();

        // 1960-01-01 00:00:00
        let start_ts = get_sas_epoch().and_utc().timestamp();

        let n_seconds: f64 = end_ts as f64 - start_ts as f64;

        // insert the timestamp at offset 164
        bytes[164..172].copy_from_slice(&n_seconds.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let ts = header.get_creation_timestamp_from_header();
        assert_approx_eq!(ts, n_seconds);
    }

    #[test]
    fn can_get_modification_timestamp_offset_when_a1_eq_0() {
        let bytes = vec![0_u8; 8192];

        let header = SasHeaderBinary::new(&bytes);
        let a1 = header.get_a1();
        assert_eq!(a1, 0);

        let ts_offset = header.get_modification_ts_offset();
        assert_eq!(ts_offset, 172 + a1 as usize);
    }

    #[test]
    fn can_get_modification_timestamp_offset_when_a1_eq_4() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        let header = SasHeaderBinary::new(&bytes);
        let a1 = header.get_a1();
        assert_eq!(a1, 4);

        let ts_offset = header.get_modification_ts_offset();
        assert_eq!(ts_offset, 172 + a1 as usize);
    }

    #[test]
    fn can_get_header_length() {
        let bytes = vec![0_u8; 8192];
        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 0);
    }

    #[test]
    fn can_get_header_length_when_a1_eq_0_and_byte_200_eq_9() {
        let mut bytes = vec![0_u8; 8192];
        bytes[200] = 9; // This does not affect header length

        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 0);

        bytes[196] = 9; // This makes header length = 9
        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 9);
    }

    #[test]
    fn can_get_header_length_when_a1_eq_4_and_byte_200_eq_9() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4
        bytes[200] = 9; // This makes header length = 9

        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 9);

        bytes[200] = 0; // This resets header length to 0
        bytes[196] = 9; // This does not affect header length

        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 0);
    }

    #[test]
    fn can_get_header_len_when_a1_eq_0_and_len_forced_to_be_8192() {
        let mut bytes = vec![0_u8; 8192];
        bytes[196..200].copy_from_slice(&8192_i32.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 8192);
    }

    #[test]
    fn can_get_header_len_when_a1_eq_4_and_len_forced_to_be_8192() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        // Since a1=4, the header length is at offset 200,
        // so this won't affect the header length:
        bytes[196..200].copy_from_slice(&8192_i32.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 0);

        // Now set the header length at offset 200
        bytes[200..204].copy_from_slice(&8192_i32.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let header_len = header.get_header_length_from_header();

        assert_eq!(header_len, 8192);
    }

    #[test]
    fn can_get_page_size_from_header_when_a1_eq_0() {
        let mut bytes = vec![0_u8; 8192];
        bytes[200..204].copy_from_slice(&20_i32.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let page_size = header.get_page_size_from_header();

        assert_eq!(page_size, 20);
    }

    #[test]
    fn can_get_page_size_from_header_when_a1_eq_4() {
        let mut bytes = vec![0_u8; 8192];
        bytes[35] = 0x33; // This makes a1=4

        // Since a1=4, the page size is at offset 204,
        // so this won't affect the page size:
        bytes[200..204].copy_from_slice(&20_i32.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let page_size = header.get_page_size_from_header();

        assert_eq!(page_size, 0);

        // Now set the page size at offset 204
        bytes[204..208].copy_from_slice(&20_i32.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let page_size = header.get_page_size_from_header();

        assert_eq!(page_size, 20);
    }

    fn test_page_count_a2_0(desired_page_count: i32, a1: u8) {
        let mut bytes = vec![0_u8; 8192];
        let a1_byte = if a1 == 4 { 0x33 } else { 0 };

        bytes[35] = a1_byte;

        bytes[204 + a1 as usize..208 + a1 as usize]
            .copy_from_slice(&desired_page_count.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);

        let page_count_len = header.get_page_count_len();
        println!("page_count_len: {}", page_count_len);

        let a2_byte = bytes[32];
        println!("a2_byte: {}", a2_byte);

        let page_count = header.get_page_count_from_header();

        assert_eq!(page_count, desired_page_count as usize);
    }

    fn test_page_count_a2_4(desired_page_count: i64, a1: u8) {
        let mut bytes = vec![0_u8; 8192];
        let a1_byte = if a1 == 4 { 0x33 } else { 0 };
        let a2_byte = 0x33_u8;

        bytes[35] = a1_byte;
        bytes[32] = a2_byte;

        bytes[204 + a1 as usize..212 + a1 as usize]
            .copy_from_slice(&desired_page_count.to_le_bytes());

        let header = SasHeaderBinary::new(&bytes);
        let page_count = header.get_page_count_from_header();

        assert_eq!(page_count, desired_page_count as usize);
    }

    #[test]
    fn can_get_page_count_from_header_when_a1_0_a2_0_pagesize_20() {
        test_page_count_a2_0(20, 0);
    }

    #[test]
    fn can_get_page_count_from_header_when_a1_4_a2_0_pagesize_30() {
        test_page_count_a2_0(30, 4);
    }

    #[test]
    fn can_get_page_count_from_header_when_a1_0_a2_4_pagesize_40() {
        test_page_count_a2_4(40, 0);
    }

    #[test]
    fn can_get_page_count_from_header_when_a1_4_a2_4_pagesize_50() {
        test_page_count_a2_4(50, 4);
    }

    #[test]
    fn can_get_sas_release_from_header() {
        let header = header_from_test_file();
        let sas_release = header.get_sas_release_from_header();
        let expected = "9.0401M1".to_string();
        assert_eq!(sas_release, expected);
    }

    #[test]
    fn can_get_host_sas_server_type_from_header() {
        let header = header_from_test_file();
        let host_sas_server_type = header.get_host_sas_server_type_from_header();
        let expected = "X64_8PRO".to_string();
        assert_eq!(host_sas_server_type, expected);
    }

    #[test]
    fn can_get_os_version_number_from_header_when_not_unix() {
        let header = header_from_test_file();
        let os_version_number = header.get_os_version_number_from_header();
        let expected = None;
        assert_eq!(os_version_number, expected);
    }

    fn test_os_version_number(a1: u8, a2: u8) {
        let mut bytes = vec![0_u8; 8192];
        bytes[39] = b'1'; // Unix

        if a1 == 4 {
            fix_bytes_for_a1_4(&mut bytes);
        }

        if a2 == 4 {
            fix_bytes_for_a2_4(&mut bytes);
        }

        let start: usize = 240 + a1 as usize + a2 as usize;
        let end: usize = start + 16;
        let range = start..end;
        let padded_replacement_value = [
            // "UNIX_IBM" padded with null bytes
            b'U', b'N', b'I', b'X', b'_', b'I', b'B', b'M', 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        bytes[range].copy_from_slice(&padded_replacement_value);

        let header = SasHeaderBinary::new(&bytes);
        let os_version_number = header.get_os_version_number_from_header();
        let expected = Some("UNIX_IBM".to_string());
        assert_eq!(os_version_number, expected);
    }

    #[test]
    fn can_get_os_version_number_from_header_when_unix_a1_0_a2_0() {
        test_os_version_number(0, 0);
    }

    #[test]
    fn can_get_os_version_number_from_header_when_unix_a1_4_a2_0() {
        test_os_version_number(4, 0);
    }

    #[test]
    fn can_get_os_version_number_from_header_when_unix_a1_0_a2_4() {
        test_os_version_number(0, 4);
    }

    #[test]
    fn can_get_os_version_number_from_header_when_unix_a1_4_a2_4() {
        test_os_version_number(4, 4);
    }

    #[test]
    fn can_get_os_maker_or_version_from_header() {
        let mut bytes = vec![0_u8; 8192];
        let a1 = 4;
        let a2 = 4;

        fix_bytes_for_a1_4(bytes.as_mut_slice());
        fix_bytes_for_a2_4(bytes.as_mut_slice());

        let start: usize = 256 + a1 as usize + a2 as usize;
        let end: usize = start + 16;
        let range = start..end;

        let padded_replacement_value = [
            // "IBM" padded with null bytes
            b'I', b'B', b'M', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        bytes[range].copy_from_slice(&padded_replacement_value);

        let header = SasHeaderBinary::new(&bytes);
        let os_version = header.get_os_maker_or_version_from_header();
        let expected = OsMaker::from_ascii("IBM".to_string()).unwrap();
        assert_eq!(os_version, Some(expected));
    }
}
