#[derive(Debug, Clone)]
pub struct SasConstants {
    pub magic_number: [u8; 32],
    pub file_header_size_32_bit: u32,
    pub file_header_size_64_bit: u32,
    pub default_page_size: u32,
}

impl SasConstants {
    pub fn new() -> Self {
        let magic_number = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea,
            0x81, 0x60, 0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c,
            0x18, 0x1f, 0x10, 0x11,
        ];

        SasConstants {
            magic_number,
            file_header_size_32_bit: 1024,
            file_header_size_64_bit: 8192,
            default_page_size: 4096,
        }
    }
}

impl Default for SasConstants {
    fn default() -> Self {
        Self::new()
    }
}
