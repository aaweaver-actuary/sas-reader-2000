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
    /// Returns a new `SasMetadataBuilder` with default values.
    ///
    /// # Examples
    /// ```
    /// use sas_reader_2000::sas::metadata::SasMetadata;
    /// use sas_reader_2000::sas::endianness::Endianness;
    /// use sas_reader_2000::sas::alignment::Alignment;
    /// use sas_reader_2000::sas::compression::Compression;
    ///
    /// let meta = SasMetadata::builder().build().unwrap();
    /// assert_eq!(meta.endianness(), Endianness::Big);
    /// assert_eq!(meta.alignment(), Alignment::default());
    /// assert_eq!(meta.compression(), Compression::None);
    /// ```
    pub fn builder() -> SasMetadataBuilder {
        SasMetadataBuilder::default()
    }

    /// Updates the endianness of the metadata.
    ///
    /// # Examples
    /// ```
    /// use sas_reader_2000::sas::metadata::SasMetadata;
    /// use sas_reader_2000::sas::endianness::Endianness;
    ///
    /// let mut meta = SasMetadata::builder().build().unwrap();
    /// assert_eq!(meta.endianness(), Endianness::Big);
    /// meta.update_endianness(Endianness::Little);
    /// assert_eq!(meta.endianness(), Endianness::Little);
    /// ```
    pub fn update_endianness(&mut self, endianness: Endianness) {
        self.endianness = endianness;
    }

    /// Updates the alignment of the metadata.
    ///
    /// # Examples
    /// ```
    /// use sas_reader_2000::sas::metadata::SasMetadata;
    /// use sas_reader_2000::sas::alignment::Alignment;
    ///
    /// let mut meta = SasMetadata::builder().build().unwrap();
    /// assert_eq!(meta.alignment(), Alignment::default());
    /// meta.update_alignment(Alignment::new(8, 8));
    /// assert_eq!(meta.alignment(), Alignment::new(8, 8));
    /// ```
    pub fn update_alignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    pub fn update_compression(&mut self, compression: Compression) {
        self.compression = compression;
    }

    pub fn endianness(&self) -> Endianness {
        self.endianness
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    pub fn compression(&self) -> Compression {
        self.compression
    }
}

impl SasMetadataBuilder {
    pub fn default() -> Self {
        SasMetadataBuilder {
            endianness: Option<Endianness::Big>,
            alignment: Option<Alignment::default()>,
            compression: Option<Compression::None>,
        }
    }
}
