use crate::{character_set::CharacterSet, command, constants::LF, Image};

pub enum QRCodeModel {
    Model1,
    Model2,
    Micro,
}

impl Default for QRCodeModel {
    fn default() -> Self {
        QRCodeModel::Model2
    }
}

pub enum QRCodeCellSize {
    Size1,
    Size2,
    Size3,
    Size4,
    Size5,
    Size6,
    Size7,
    Size8,
}

pub enum QRCodeCorrection {
    L,
    M,
    Q,
    H,
}

pub struct QRCodeOptions {
    model: QRCodeModel,
    cell_size: QRCodeCellSize,
    correction: QRCodeCorrection,
}

impl Default for QRCodeOptions {
    fn default() -> Self {
        Self {
            model: QRCodeModel::Model2,
            cell_size: QRCodeCellSize::Size3,
            correction: QRCodeCorrection::L,
        }
    }
}

pub enum HriPosition {
    None,
    Top,
    Bottom,
    TopBottom,
}

pub enum TextFont {
    A,
    B,
    C,
    D,
    E,
    SpecialA,
    SpecialB,
}

pub enum BarcodeWidth {
    Width1,
    Width2,
    Width3,
    Width4,
    Width5,
    Width6,
}

pub struct BarcodeOptions {
    pub hri_position: HriPosition,
    pub hri_font: TextFont,
    pub width: BarcodeWidth,
    pub height: u8,
}

impl Default for BarcodeOptions {
    fn default() -> Self {
        Self {
            hri_position: HriPosition::None,
            hri_font: TextFont::A,
            width: BarcodeWidth::Width3,
            height: 162,
        }
    }
}

pub enum BarcodeType {
    UPCA,
    UPCE,
    JAN13,
    JAN8,
    CODE39,
    ITF,
    CODABAR,
    CODE93,
    CODE128,
    GS1128,
    GS1DataBarOmnidirectional,
    GS1DataBarTruncated,
    GS1DataBarLimited,
    GS1DataBarExpanded,
}

pub enum CashDrawerPin {
    TwoPin,
    FivePin,
}

pub enum TextUnderline {
    OneDotThick,
    TwoDotThick,
    None,
}

pub enum Align {
    Left,
    Center,
    Right,
}

impl Default for Align {
    fn default() -> Self {
        Align::Left
    }
}

pub enum TextSize {
    Size1,
    Size2,
    Size3,
    Size4,
    Size5,
    Size6,
    Size7,
    Size8,
}

pub trait Printer {
    fn set_character_set(&mut self, set: CharacterSet) -> &mut Self;
    fn set_text_font(&mut self, font: TextFont) -> &mut Self;
    fn set_text_bold(&mut self, bold: bool) -> &mut Self;
    fn set_text_size(&mut self, width: TextSize, height: TextSize) -> &mut Self;
    fn set_text_underline(&mut self, underline: TextUnderline) -> &mut Self;
    fn set_text_normal(&mut self) -> &mut Self;
    fn set_align(&mut self, align: Align) -> &mut Self;
    fn invert(&mut self, enabled: bool) -> &mut Self;
    fn text(&mut self, data: &str) -> &mut Self;
    fn raw(&mut self, data: &[u8]) -> &mut Self;
    fn new_line(&mut self) -> &mut Self;
    fn cut(&mut self, partial: bool) -> &mut Self;
    fn image(&mut self, image: Image) -> &mut Self;
    fn qrcode(&mut self, data: &str, options: QRCodeOptions) -> &mut Self;
    fn barcode(&mut self, data: &str, r#type: BarcodeType, options: BarcodeOptions) -> &mut Self;
    fn cash_draw(&mut self, pin: CashDrawerPin) -> &mut Self;
    fn initialize(&mut self) -> &mut Self;
    fn get_data(&self) -> Vec<u8>;
    fn clear(&mut self) -> &mut Self;
    fn repeat(&mut self, times: usize, f: fn(&mut Self) -> &mut Self) -> &mut Self {
        for _ in 0..times {
            f(self);
        }

        self
    }
}

pub struct BasePrinter {
    cmds: Vec<u8>,
    character_set: CharacterSet,
}

impl Default for BasePrinter {
    fn default() -> Self {
        Self {
            cmds: vec![],
            character_set: CharacterSet::default(),
        }
    }
}

impl Printer for BasePrinter {
    fn set_character_set(&mut self, set: CharacterSet) -> &mut Self {
        self.cmds.extend_from_slice(&command::character_set(&set));
        self.character_set = set;
        self
    }

    fn set_text_font(&mut self, font: TextFont) -> &mut Self {
        let n = match font {
            TextFont::A => 0,
            TextFont::B => 1,
            TextFont::C => 2,
            TextFont::D => 3,
            TextFont::E => 4,
            TextFont::SpecialA => 97,
            TextFont::SpecialB => 98,
        };

        self.cmds.extend_from_slice(&command::text_font(n));

        self
    }

    fn set_text_bold(&mut self, bold: bool) -> &mut Self {
        self.cmds
            .extend_from_slice(&command::text_bold(if bold { 1 } else { 0 }));
        self
    }

    fn set_text_size(&mut self, width: TextSize, height: TextSize) -> &mut Self {
        let w = match width {
            TextSize::Size1 => 0 * 16,
            TextSize::Size2 => 1 * 16,
            TextSize::Size3 => 2 * 16,
            TextSize::Size4 => 3 * 16,
            TextSize::Size5 => 4 * 16,
            TextSize::Size6 => 5 * 16,
            TextSize::Size7 => 6 * 16,
            TextSize::Size8 => 7 * 16,
        };

        let h = match height {
            TextSize::Size1 => 0,
            TextSize::Size2 => 1,
            TextSize::Size3 => 2,
            TextSize::Size4 => 3,
            TextSize::Size5 => 4,
            TextSize::Size6 => 5,
            TextSize::Size7 => 6,
            TextSize::Size8 => 7,
        };

        let n = w + h;

        self.cmds.extend_from_slice(&command::text_size(n));

        self
    }

    fn set_text_underline(&mut self, underline: TextUnderline) -> &mut Self {
        let n = match underline {
            TextUnderline::OneDotThick => 1,
            TextUnderline::TwoDotThick => 2,
            TextUnderline::None => 0,
        };

        self.cmds.extend_from_slice(&command::text_underline(n));

        self
    }

    fn set_text_normal(&mut self) -> &mut Self {
        self.cmds.extend_from_slice(&command::text_mode(0));
        self
    }

    fn set_align(&mut self, align: Align) -> &mut Self {
        let n = match align {
            Align::Left => 0,
            Align::Center => 1,
            Align::Right => 2,
        };

        self.cmds.extend_from_slice(&command::alignment(n));

        self
    }

    fn invert(&mut self, enabled: bool) -> &mut Self {
        self.cmds
            .extend_from_slice(&command::invert(if enabled { 1 } else { 0 }));
        self
    }

    fn text(&mut self, data: &str) -> &mut Self {
        self.cmds
            .extend(iconv::encode(data, (&self.character_set).into()).unwrap());
        self
    }

    fn raw(&mut self, data: &[u8]) -> &mut Self {
        self.cmds.extend_from_slice(data);
        self
    }

    fn new_line(&mut self) -> &mut Self {
        self.cmds.push(LF);
        self
    }

    fn cut(&mut self, partial: bool) -> &mut Self {
        self.cmds
            .extend_from_slice(&command::cut(if partial { 49 } else { 48 }, None));
        self
    }

    fn image(&mut self, image: Image) -> &mut Self {
        let width_ceiled = (image.width() as f64 / 8.0).ceil() as u16;
        let xl = width_ceiled as u8;
        let xh = (width_ceiled >> 8) as u8;

        let height = image.height();
        let yl = height as u8;
        let yh = (height >> 8) as u8;

        self.cmds
            .extend_from_slice(&command::image(0, xl, xh, yl, yh, &image.to_raster()));

        self
    }

    fn qrcode(&mut self, data: &str, options: QRCodeOptions) -> &mut Self {
        let model_value = match options.model {
            QRCodeModel::Model1 => 49,
            QRCodeModel::Model2 => 50,
            QRCodeModel::Micro => 51,
        };

        self.cmds
            .extend_from_slice(&command::qr_code_model(model_value, None));

        let cell_size_value = match options.cell_size {
            QRCodeCellSize::Size1 => 1,
            QRCodeCellSize::Size2 => 2,
            QRCodeCellSize::Size3 => 3,
            QRCodeCellSize::Size4 => 4,
            QRCodeCellSize::Size5 => 5,
            QRCodeCellSize::Size6 => 6,
            QRCodeCellSize::Size7 => 7,
            QRCodeCellSize::Size8 => 8,
        };

        self.cmds
            .extend_from_slice(&command::qr_code_cell_size(cell_size_value));

        let correction_value = match options.correction {
            QRCodeCorrection::L => 48,
            QRCodeCorrection::M => 49,
            QRCodeCorrection::Q => 50,
            QRCodeCorrection::H => 51,
        };

        self.cmds
            .extend_from_slice(&command::qr_code_correction_level(correction_value));

        let encoded = iconv::encode(data, (&CharacterSet::Pc437Usa).into()).unwrap();

        let encoded_length = (encoded.len() + 3) as u16;
        let pl = encoded_length as u8;
        let ph = (encoded_length >> 8) as u8;

        self.cmds
            .extend_from_slice(&command::qr_code_store(pl, ph, &encoded));

        self.cmds.extend_from_slice(&command::qr_code_print());

        self
    }

    fn barcode(&mut self, data: &str, r#type: BarcodeType, options: BarcodeOptions) -> &mut Self {
        let hri_position_value = match options.hri_position {
            HriPosition::None => 0,
            HriPosition::Top => 1,
            HriPosition::Bottom => 2,
            HriPosition::TopBottom => 3,
        };

        self.cmds
            .extend_from_slice(&command::barcode_hri_position(hri_position_value));

        let hri_font_value = match options.hri_font {
            TextFont::A => 0,
            TextFont::B => 1,
            TextFont::C => 2,
            TextFont::D => 3,
            TextFont::E => 4,
            TextFont::SpecialA => 97,
            TextFont::SpecialB => 98,
        };

        self.cmds
            .extend_from_slice(&command::barcode_hri_font(hri_font_value));

        let width_value = match options.width {
            BarcodeWidth::Width1 => 1,
            BarcodeWidth::Width2 => 2,
            BarcodeWidth::Width3 => 3,
            BarcodeWidth::Width4 => 4,
            BarcodeWidth::Width5 => 5,
            BarcodeWidth::Width6 => 6,
        };

        self.cmds
            .extend_from_slice(&command::barcode_width(width_value));

        self.cmds
            .extend_from_slice(&command::barcode_height(options.height));

        let type_value = match r#type {
            BarcodeType::UPCA => 65,
            BarcodeType::UPCE => 66,
            BarcodeType::JAN13 => 67,
            BarcodeType::JAN8 => 68,
            BarcodeType::CODE39 => 69,
            BarcodeType::ITF => 70,
            BarcodeType::CODABAR => 71,
            BarcodeType::CODE93 => 72,
            BarcodeType::CODE128 => 73,
            BarcodeType::GS1128 => 74,
            BarcodeType::GS1DataBarOmnidirectional => 75,
            BarcodeType::GS1DataBarTruncated => 76,
            BarcodeType::GS1DataBarLimited => 77,
            BarcodeType::GS1DataBarExpanded => 78,
        };

        let encoded = iconv::encode(data, (&CharacterSet::Pc437Usa).into()).unwrap();

        self.cmds.extend_from_slice(&command::barcode_print(
            type_value,
            encoded.len() as u8,
            &encoded,
        ));

        self
    }

    fn cash_draw(&mut self, pin: CashDrawerPin) -> &mut Self {
        let m = match pin {
            CashDrawerPin::TwoPin => 0,
            CashDrawerPin::FivePin => 1,
        };

        self.cmds
            .extend_from_slice(&command::cash_draw(m, 0x19, 0x78));

        self
    }

    fn initialize(&mut self) -> &mut Self {
        self.cmds.extend_from_slice(&command::initialize());
        self
    }

    fn clear(&mut self) -> &mut Self {
        self.cmds.clear();
        self
    }

    fn get_data(&self) -> Vec<u8> {
        self.cmds.clone()
    }
}
