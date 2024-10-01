#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Compression {
    None = 0,
    Truncated = 1,
    Rle = 4,
}
