#[derive(Debug)]
pub enum PageType {
    Meta,
    Data,
    Mix,
    AMD,
    Unknown(u16),
}

impl PageType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => PageType::Meta,
            256 => PageType::Data,
            512 => PageType::Mix,
            1024 => PageType::AMD,
            _ => PageType::Unknown(value),
        }
    }
}
