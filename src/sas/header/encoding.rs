#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Encoding {
    Utf8,
    UsAscii,
    Iso8859_1,
    Iso8859_2,
    Iso8859_3,
    Iso8859_4,
    Iso8859_5,
    Iso8859_6,
    Iso8859_7,
    Iso8859_8,
    Iso8859_9,
    Iso8859_11,
    Iso8859_15,
    Cp437,
    Cp850,
    Cp852,
    Cp857,
    Cp858,
    Cp862,
    Cp864,
    Cp865,
    Cp866,
    Cp869,
    Cp874,
    Cp921,
    Cp922,
    Cp1129,
    Cp720,
    Cp737,
    Cp775,
    Cp860,
    Cp863,
    Windows1250,
    Windows1251,
    Windows1252,
    Windows1253,
    Windows1254,
    Windows1255,
    Windows1256,
    Windows1257,
    Windows1258,
    Macroman,
    Macarabic,
    Machebrew,
    Macgreek,
    Macthai,
    Macturkish,
    Macukraine,
    Cp950,
    EucTw,
    Big5,
    Gb18030,
    Windows936,
    Cp1381,
    EucJp,
    Cp949,
    Cp942,
    Cp932,
    EucKr,
    Maciceland,
    Iso2022Jp,
    Iso2022Kr,
    Iso2022Cn,
    Iso2022CnExt,
    SasDefaultStringEncoding,
    Iso8859_14,
    Iso8859_13,
    Maccroatian,
    Maccyrillic,
    Macromania,
    ShiftJisx0213,
}

impl Encoding {
    pub fn from_u8(value: u8) -> Result<Self, String> {
        let output = match value {
            0 => Encoding::Windows1252,
            20 => Encoding::Utf8,
            28 => Encoding::UsAscii,
            29 => Encoding::Iso8859_1,
            30 => Encoding::Iso8859_2,
            31 => Encoding::Iso8859_3,
            32 => Encoding::Iso8859_4,
            33 => Encoding::Iso8859_5,
            34 => Encoding::Iso8859_6,
            35 => Encoding::Iso8859_7,
            36 => Encoding::Iso8859_8,
            37 => Encoding::Iso8859_9,
            39 => Encoding::Iso8859_11,
            40 => Encoding::Iso8859_15,
            41 => Encoding::Cp437,
            42 => Encoding::Cp850,
            43 => Encoding::Cp852,
            44 => Encoding::Cp857,
            45 => Encoding::Cp858,
            46 => Encoding::Cp862,
            47 => Encoding::Cp864,
            48 => Encoding::Cp865,
            49 => Encoding::Cp866,
            50 => Encoding::Cp869,
            51 => Encoding::Cp874,
            52 => Encoding::Cp921,
            53 => Encoding::Cp922,
            54 => Encoding::Cp1129,
            55 => Encoding::Cp720,
            56 => Encoding::Cp737,
            57 => Encoding::Cp775,
            58 => Encoding::Cp860,
            59 => Encoding::Cp863,
            60 => Encoding::Windows1250,
            61 => Encoding::Windows1251,
            62 => Encoding::Windows1252,
            63 => Encoding::Windows1253,
            64 => Encoding::Windows1254,
            65 => Encoding::Windows1255,
            66 => Encoding::Windows1256,
            67 => Encoding::Windows1257,
            68 => Encoding::Windows1258,
            69 => Encoding::Macroman,
            70 => Encoding::Macarabic,
            71 => Encoding::Machebrew,
            72 => Encoding::Macgreek,
            73 => Encoding::Macthai,
            75 => Encoding::Macturkish,
            76 => Encoding::Macukraine,
            118 => Encoding::Cp950,
            119 => Encoding::EucTw,
            123 => Encoding::Big5,
            125 => Encoding::Gb18030,
            126 => Encoding::Windows936,
            128 => Encoding::Cp1381,
            134 => Encoding::EucJp,
            136 => Encoding::Cp949,
            137 => Encoding::Cp942,
            138 => Encoding::Cp932,
            140 => Encoding::EucKr,
            141 => Encoding::Cp949,
            142 => Encoding::Cp949,
            163 => Encoding::Maciceland,
            167 => Encoding::Iso2022Jp,
            168 => Encoding::Iso2022Kr,
            169 => Encoding::Iso2022Cn,
            172 => Encoding::Iso2022CnExt,
            204 => Encoding::Windows1252,
            205 => Encoding::Gb18030,
            227 => Encoding::Iso8859_14,
            242 => Encoding::Iso8859_13,
            245 => Encoding::Maccroatian,
            246 => Encoding::Maccyrillic,
            247 => Encoding::Macromania,
            248 => Encoding::ShiftJisx0213,
            _ => return Err("Invalid encoding code".to_string()),
        };
        Ok(output)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_get_encoding_from_code() {
        let encoding = Encoding::from_u8(0);
        assert_eq!(encoding, Ok(Encoding::Windows1252));
    }

    #[test]
    fn test_code_for_utf8() {
        let encoding = Encoding::from_u8(20);
        assert_eq!(encoding, Ok(Encoding::Utf8));
    }

    #[test]
    fn test_code_for_iso8859_1() {
        // AKA Latin-1
        let encoding = Encoding::from_u8(29);
        assert_eq!(encoding, Ok(Encoding::Iso8859_1));
    }

    #[test]
    fn can_get_encoding_from_invalid_code() {
        let encoding = Encoding::from_u8(250);
        assert_eq!(encoding, Err("Invalid encoding code".to_string()));
    }
}
