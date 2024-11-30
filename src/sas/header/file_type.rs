#[derive(Debug, PartialEq)]
pub enum FileType {
    Data,
    Catalog,
}

impl FileType {
    pub fn from_str(value: &str) -> Result<FileType, String> {
        match value.to_lowercase().trim() {
            "data" => Ok(FileType::Data),
            "catalog" => Ok(FileType::Catalog),
            _ => {
                let message = format!(
                    "Unknown file type: {}. Expected either 'data' or 'catalog'.",
                    value
                );
                Err(message)
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_make_file_type_enum() {
        let file_type1 = FileType::Data;
        let file_type2 = FileType::from_str("data").unwrap();

        assert_eq!(file_type1, file_type2);
    }

    #[test]
    fn test_file_type_from_str_when_data() {
        let file_type = FileType::from_str("data").unwrap();
        assert_eq!(file_type, FileType::Data);
    }

    #[test]
    fn test_file_type_from_str_when_catalog() {
        let file_type = FileType::from_str("catalog").unwrap();
        assert_eq!(file_type, FileType::Catalog);
    }

    #[test]
    #[should_panic(expected = "Unknown file type: invalid. Expected either 'data' or 'catalog'.")]
    fn test_file_type_from_str_when_unknown() {
        FileType::from_str("invalid").unwrap();
    }
}
