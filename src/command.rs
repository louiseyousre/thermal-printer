use crate::{
    character_set::CharacterSet,
    constants::{ESC, GS},
};

/// Select justification
///
/// # Arguments
///
/// * `n` - Justification value
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = alignment(0);
/// ```
pub fn alignment(n: u8) -> [u8; 3] {
    [ESC, 0x61, n]
}

/// Set barcode height
///
/// # Arguments
///
/// * `n` - Barcode height value
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = barcode_height(100);
/// ```
pub fn barcode_height(n: u8) -> [u8; 3] {
    [GS, 0x68, n]
}

/// Select font for HRI characters
///
/// # Arguments
///
/// * `n` - Font of HRI characters
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = barcode_hri_font(0);
/// ```
pub fn barcode_hri_font(n: u8) -> [u8; 3] {
    [GS, 0x66, n]
}

/// Select print position of HRI characters
///
/// # Arguments
///
/// * `n` - Print position value
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = barcode_hri_position(1);
/// ```
pub fn barcode_hri_position(n: u8) -> [u8; 3] {
    [GS, 0x48, n]
}

/// Print barcode
///
/// # Arguments
///
/// * `m` - Barcode system
/// * `n` - Barcode length
/// * `data` - Barcode data
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = barcode_print(73, 10, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn barcode_print(m: u8, n: u8, data: &[u8]) -> Vec<u8> {
    let mut command = vec![GS, 0x6b, m, n];
    command.extend_from_slice(data);
    command
}

/// Set barcode width
///
/// # Arguments
///
/// * `n` - Barcode width value
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = barcode_width(2);
/// ```
pub fn barcode_width(n: u8) -> [u8; 3] {
    [GS, 0x77, n]
}

/// Generate pulse
///
/// # Arguments
///
/// * `m` - Drawer number
/// * `t1` - On time
/// * `t2` - Off time
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = cash_draw(0, 120, 240);
/// ```
pub fn cash_draw(m: u8, t1: u8, t2: u8) -> [u8; 5] {
    [ESC, 0x70, m, t1, t2]
}

/// Select character code table
///
/// # Arguments
///
/// * `n` - Character code table number
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = character_code_table(0);
/// ```
pub fn character_code_table(n: u8) -> [u8; 3] {
    [ESC, 0x74, n]
}

/// Select cut mode and cut paper
///
/// # Arguments
///
/// * `m` - Cut mode
/// * `n` - Cut type (optional, only for Function B, C, D)
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command_a = cut(0);
/// let command_b = cut(0, 1);
/// ```
pub fn cut(m: u8, n: Option<u8>) -> Vec<u8> {
    let mut cmd = vec![GS, 0x56, m];
    if let Some(val) = n {
        cmd.push(val);
    }
    cmd
}

/// Print raster bit image
///
/// # Arguments
///
/// * `m` - Printing mode
/// * `xl` - Width of the image (LSB)
/// * `xh` - Width of the image (MSB)
/// * `yl` - Height of the image (LSB)
/// * `yh` - Height of the image (MSB)
/// * `data` - Image data
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let image_data: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Example image data
/// let command = image(0, 0, 1, 0, 1, &image_data);
/// ```
pub fn image(m: u8, xl: u8, xh: u8, yl: u8, yh: u8, data: &[u8]) -> Vec<u8> {
    let mut base = vec![GS, 0x76, 0x30, m, xl, xh, yl, yh];
    base.extend_from_slice(data);
    base
}

/// Initialize printer
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = initialize();
/// ```
pub fn initialize() -> [u8; 2] {
    [ESC, 0x40]
}

/// Select an international character set
///
/// # Arguments
///
/// * `n` - International character set number
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = international_character_set(0);
/// ```
pub fn international_character_set(n: u8) -> [u8; 3] {
    [ESC, 0x52, n]
}

/// Specify/cancel white/black inverted printing
///
/// # Arguments
///
/// * `n` - Specify/cancel inverted printing (0 or 1)
///
/// # Returns
///
/// An array representing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = invert(1);
/// ```
pub fn invert(n: u8) -> [u8; 3] {
    [GS, 0x42, n]
}

/// QR Code: Set the size of module
///
/// # Arguments
///
/// * `n` - Size of the module
///
/// # Returns
///
/// An array containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = qr_code_cell_size(5);
/// ```
pub fn qr_code_cell_size(n: u8) -> [u8; 8] {
    [GS, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, n]
}

/// QR Code: Select the error correction level
///
/// # Arguments
///
/// * `n` - Error correction level (default value: 48)
///
/// # Returns
///
/// An array containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = qr_code_correction_level(48);
/// ```
pub fn qr_code_correction_level(n: u8) -> [u8; 8] {
    [GS, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x45, n]
}

/// QR Code: Select the model
///
/// # Arguments
///
/// * `n1` - Model (default value: 50)
/// * `n2` - Version (default value: 0)
///
/// # Returns
///
/// An array containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = qr_code_model(50, 0);
/// ```
pub fn qr_code_model(n1: u8, n2: Option<u8>) -> [u8; 9] {
    [GS, 0x28, 0x6b, 0x04, 0x00, 0x31, 0x41, n1, n2.unwrap_or(0)]
}

/// QR Code: Print the symbol data in the symbol storage area
///
/// # Returns
///
/// An array containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = qr_code_print();
/// ```
pub fn qr_code_print() -> [u8; 8] {
    [GS, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x51, 48]
}

/// QR Code: Store the data in the symbol storage area
///
/// # Arguments
///
/// * `pl` - Length of data (LSB)
/// * `ph` - Length of data (MSB)
/// * `data` - Data to be stored
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let data: Vec<u8> = vec![0x41, 0x42, 0x43]; // Example data
/// let command = qr_code_store(0x03, 0x00, &data);
/// ```
pub fn qr_code_store(pl: u8, ph: u8, data: &[u8]) -> Vec<u8> {
    let mut command = vec![GS, 0x28, 0x6b, pl, ph, 0x31, 0x50, 0x30];
    command.extend_from_slice(data);
    command
}

/// Turn emphasized mode on/off
///
/// # Arguments
///
/// * `n` - Mode (0: off, 1: on)
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = text_bold(1);
/// ```
pub fn text_bold(n: u8) -> [u8; 3] {
    [ESC, 0x45, n]
}

/// Select character font
///
/// # Arguments
///
/// * `n` - Font type
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = text_font(1);
/// ```
pub fn text_font(n: u8) -> [u8; 3] {
    [ESC, 0x4d, n]
}

/// Select print mode(s)
///
/// # Arguments
///
/// * `n` - Print mode bitmask
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = text_mode(0b00000011); // Example: double height and double width
/// ```
pub fn text_mode(n: u8) -> [u8; 3] {
    [ESC, 0x21, n]
}

/// Select character size
///
/// # Arguments
///
/// * `n` - Character size
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = text_size(0b00000011); // Example: double height and double width
/// ```
pub fn text_size(n: u8) -> [u8; 3] {
    [GS, 0x21, n]
}

/// Turn underline mode on/off
///
/// # Arguments
///
/// * `n` - Mode (0: off, 1: on)
///
/// # Returns
///
/// A vector containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = text_underline(1);
/// ```
pub fn text_underline(n: u8) -> [u8; 3] {
    [ESC, 0x2d, n]
}

/// Select character set
///
/// # Arguments
///
/// * `set` - Character set to select
///
/// # Returns
///
/// A slice containing the command bytes
///
/// # Example
///
/// ```ignore
/// let command = character_set(CharacterSet::Japan);
/// ```
pub fn character_set(set: &CharacterSet) -> [u8; 3] {
    match set {
        CharacterSet::Pc437Usa => character_code_table(0),
        CharacterSet::Pc850Multilingual => character_code_table(2),
        CharacterSet::Pc860Portuguese => character_code_table(3),
        CharacterSet::Pc863CanadianFrench => character_code_table(4),
        CharacterSet::Pc865Nordic => character_code_table(5),
        CharacterSet::Pc851Greek => character_code_table(11),
        CharacterSet::Pc857Turkish => character_code_table(12),
        CharacterSet::Pc737Greek => character_code_table(14),
        CharacterSet::Iso8859_7Greek => character_code_table(15),
        CharacterSet::Wpc1252 => character_code_table(16),
        CharacterSet::Pc866Cyrillic2 => character_code_table(17),
        CharacterSet::Pc852Latin2 | CharacterSet::Slovenia => character_code_table(18),
        CharacterSet::Pc858Euro => character_code_table(19),
        CharacterSet::Wpc775BalticRim => character_code_table(33),
        CharacterSet::Pc855Cyrillic => character_code_table(34),
        CharacterSet::Pc861Icelandic => character_code_table(35),
        CharacterSet::Pc862Hebrew => character_code_table(36),
        CharacterSet::Pc864Arabic => character_code_table(37),
        CharacterSet::Pc869Greek => character_code_table(38),
        CharacterSet::Iso8859_2Latin2 => character_code_table(39),
        CharacterSet::Iso8859_15Latin9 => character_code_table(40),
        CharacterSet::Pc1125Ukranian => character_code_table(44),
        CharacterSet::Wpc1250Latin2 => character_code_table(45),
        CharacterSet::Wpc1251Cyrillic => character_code_table(46),
        CharacterSet::Wpc1253Greek => character_code_table(47),
        CharacterSet::Wpc1254Turkish => character_code_table(48),
        CharacterSet::Wpc1255Hebrew => character_code_table(49),
        CharacterSet::Wpc1256Arabic => character_code_table(50),
        CharacterSet::Wpc1257BalticRim => character_code_table(51),
        CharacterSet::Wpc1258Vietnamese => character_code_table(52),
        CharacterSet::Kz1048Kazakhstan => character_code_table(53),

        CharacterSet::Japan => international_character_set(0x08),
        CharacterSet::Korea => international_character_set(0x0d),
        CharacterSet::China => international_character_set(0x0f),
        CharacterSet::HkTw => international_character_set(0x00),
    }
}
