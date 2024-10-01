/// Represents the endianness of the data in the SAS file.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Endianness {
    Big,
    Little,
}

impl Endianness {
    /// Converts a binary value to an `Endianness` value.
    ///
    /// # Arguments
    ///
    /// * `value` - The binary value to convert.
    ///
    /// # Returns
    ///
    /// An `Endianness` value if the binary value is valid, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sas_reader_2000::sas::Endianness;
    ///
    /// let endianness = Endianness::from_u8(0x00);
    /// assert_eq!(endianness, Some(Endianness::Big));
    ///
    /// let endianness = Endianness::from_u8(0x01);
    /// assert_eq!(endianness, Some(Endianness::Little));
    ///
    /// let endianness = Endianness::from_u8(0x02);
    /// assert_eq!(endianness, None);
    /// ```
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Endianness::Big),
            0x01 => Some(Endianness::Little),
            _ => None,
        }
    }
}
