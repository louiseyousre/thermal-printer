/// Character sets

pub enum CharacterSet {
    Pc437Usa,
    Pc850Multilingual,
    Pc860Portuguese,
    Pc863CanadianFrench,
    Pc865Nordic,
    Pc851Greek,
    Pc857Turkish,
    Pc737Greek,
    Iso8859_7Greek,
    Wpc1252,
    Pc866Cyrillic2,
    Pc852Latin2,
    Slovenia,
    Pc858Euro,
    Wpc775BalticRim,
    Pc855Cyrillic,
    Pc861Icelandic,
    Pc862Hebrew,
    Pc864Arabic,
    Pc869Greek,
    Iso8859_2Latin2,
    Iso8859_15Latin9,
    Pc1125Ukranian,
    Wpc1250Latin2,
    Wpc1251Cyrillic,
    Wpc1253Greek,
    Wpc1254Turkish,
    Wpc1255Hebrew,
    Wpc1256Arabic,
    Wpc1257BalticRim,
    Wpc1258Vietnamese,
    Kz1048Kazakhstan,
    Japan,
    Korea,
    China,
    HkTw,
}

impl Default for CharacterSet {
    fn default() -> Self {
        CharacterSet::Pc437Usa
    }
}

impl Into<&str> for &CharacterSet {
    fn into(self) -> &'static str {
        match self {
            CharacterSet::Pc437Usa => "CP437",
            CharacterSet::Pc850Multilingual => "CP850",
            CharacterSet::Pc860Portuguese => "CP860",
            CharacterSet::Pc863CanadianFrench => "CP863",
            CharacterSet::Pc865Nordic => "CP865",
            CharacterSet::Pc851Greek => "CP860",
            CharacterSet::Pc857Turkish => "CP857",
            CharacterSet::Pc737Greek => "CP737",
            CharacterSet::Iso8859_7Greek => "ISO-8859-7",
            CharacterSet::Wpc1252 => "CP1252",
            CharacterSet::Pc866Cyrillic2 => "CP866",
            CharacterSet::Pc852Latin2 => "CP852",
            CharacterSet::Slovenia => "CP852",
            CharacterSet::Pc858Euro => "CP858",
            CharacterSet::Wpc775BalticRim => "CP775",
            CharacterSet::Pc855Cyrillic => "CP855",
            CharacterSet::Pc861Icelandic => "CP861",
            CharacterSet::Pc862Hebrew => "CP862",
            CharacterSet::Pc864Arabic => "CP864",
            CharacterSet::Pc869Greek => "CP869",
            CharacterSet::Iso8859_2Latin2 => "ISO-8859-2",
            CharacterSet::Iso8859_15Latin9 => "ISO-8859-15",
            CharacterSet::Pc1125Ukranian => "CP1125",
            CharacterSet::Wpc1250Latin2 => "WIN1250",
            CharacterSet::Wpc1251Cyrillic => "WIN1251",
            CharacterSet::Wpc1253Greek => "WIN1253",
            CharacterSet::Wpc1254Turkish => "WIN1254",
            CharacterSet::Wpc1255Hebrew => "WIN1255",
            CharacterSet::Wpc1256Arabic => "WIN1256",
            CharacterSet::Wpc1257BalticRim => "WIN1257",
            CharacterSet::Wpc1258Vietnamese => "WIN1258",
            CharacterSet::Kz1048Kazakhstan => "RK1048",
            CharacterSet::Japan => "EUC-JP",
            CharacterSet::Korea => "EUC-KR",
            CharacterSet::China => "GB18030",
            CharacterSet::HkTw => "Big5-HKSCS",
        }
    }
}
