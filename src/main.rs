use thermal_printer::{
    Align, BarcodeOptions, BarcodeType, BarcodeWidth, BasePrinter, HriPosition, Image, Pixel,
    Printer, TextFont,
};

fn main() {
    const IMAGE: Image = Image::new(
        &[
            Pixel::new(0, 0, 0, 0),
            Pixel::new(0, 0, 0, 0),
            Pixel::new(0, 0, 0, 0),
        ],
        0,
        0,
    );

    let data = BasePrinter::default()
        .set_text_bold(true)
        .set_align(Align::Center)
        .text("Here's your barcode")
        .set_align(Align::default())
        .set_text_bold(false)
        .new_line()
        .image(IMAGE)
        .barcode(
            "123456",
            BarcodeType::CODABAR,
            BarcodeOptions {
                hri_position: HriPosition::Bottom,
                hri_font: TextFont::A,
                width: BarcodeWidth::Width1,
                height: 50,
            },
        )
        .new_line()
        .text("I hope you like my lib")
        .repeat(7, BasePrinter::new_line)
        .cut(false)
        .get_data();

    println!("{data:#?}");
}
