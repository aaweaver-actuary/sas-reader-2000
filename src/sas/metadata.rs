use crate::sas::alignment::Alignment;
use crate::sas::compression::Compression;
use crate::sas::endianness::Endianness;

use derive_builder::Builder;

#[derive(Debug, Builder, PartialEq)]
pub struct SasMetadata {
    endianness: Endianness,
    alignment: Alignment,
    compression: Compression,
}

impl SasMetadata {
    pub fn builder() -> SasMetadataBuilder {
        SasMetadataBuilder::default()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_sas_metadata_builder() {
        let metadata = SasMetadata::builder()
            .endianness(Endianness::Big)
            .alignment(Alignment::from_u8(0x00))
            .compression(Compression::None)
            .build()
            .unwrap();

        assert_eq!(metadata.endianness, Endianness::Big);
        assert_eq!(metadata.alignment, Alignment::from_u8(0x00));
        assert_eq!(metadata.compression, Compression::None);
    }
}
