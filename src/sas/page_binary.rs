/// Represents the raw binary for a page in a sas7bdat file.
pub struct PageBinary {
    pub page_type: u16,
    pub block_count: u16,
    pub subheader_count: u16,
    pub subheader_pointers: Vec<u8>, // Raw bytes of subheader pointers
    pub data: Vec<u8>,               // Raw bytes of the page content
}
