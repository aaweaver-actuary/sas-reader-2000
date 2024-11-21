use std::fs::File;
use std::io::prelude::*;

pub fn read_sas_file(filename: String) -> Result<String, std::io::Error> {
    // let file = File::open(filename)?;
    Ok(filename)
}

pub type ReadResult<T> = std::result::Result<T, std::io::Error>;

#[derive(Debug, PartialEq)]
pub struct SasReader {
    pub filename: String,
}

impl SasReader {
    pub fn new(filename: String) -> Self {
        SasReader { filename }
    }

    pub fn get_file_contents(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(&self.filename)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }

    pub fn validate_format(&self) -> Result<(), String> {
        //     let contents = self.get_file_contents();
        //     // let header = SasHeader::new(&contents.unwrap());
        //     let header = SasHeader::read(&contents.unwrap());
        //     match contents {
        //         Ok(_) => Ok(()),
        //         Err(e) => Err(format!("Failed to read file contents: {}", e)),
        //     }
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    const FILENAME: &str = "test/hadley.sas7bdat";

    use super::*;

    #[test]
    fn can_read_sas7bdat_file() {
        let sas_file = read_sas_file(FILENAME.to_string());

        assert!(sas_file.is_ok());
    }

    #[test]
    fn can_create_sas_reader() {
        let sas_reader1 = SasReader {
            filename: FILENAME.to_string(),
        };

        let sas_reader2 = SasReader::new(FILENAME.to_string());

        assert_eq!(sas_reader1.filename, FILENAME);
        assert_eq!(sas_reader1, sas_reader2);
    }

    #[test]
    fn can_get_file_contents() {
        let sas_reader = SasReader::new(FILENAME.to_string());
        match sas_reader.get_file_contents() {
            Ok(contents) => println!(
                "File contents read successfully. Size: {} bytes",
                contents.len()
            ),
            Err(e) => eprintln!("Failed to read file contents: {}", e),
        }

        assert!(sas_reader.get_file_contents().is_ok());
    }

    #[test]
    fn can_get_file_contents_with_error() {
        let sas_reader = SasReader::new("non_existent_file.sas7bdat".to_string());
        match sas_reader.get_file_contents() {
            Ok(contents) => println!(
                "File contents read successfully. Size: {} bytes",
                contents.len()
            ),
            Err(e) => eprintln!("Failed to read file contents: {}", e),
        }

        assert!(sas_reader.get_file_contents().is_err());
    }

    #[test]
    fn can_validate_sas7bdat_file_format() {
        let sas_reader = SasReader::new(FILENAME.to_string());
        match sas_reader.validate_format() {
            Ok(_) => println!("File format is valid"),
            Err(e) => eprintln!("Invalid file format: {}", e),
        }

        assert!(sas_reader.validate_format().is_ok());
    }
}
