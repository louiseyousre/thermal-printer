use crate::pixel::{Pixel, DEFAULT_PIXEL};

pub struct Image {
    data: Vec<Pixel>,
    width: u16,
    height: u16,
}

impl Image {
    pub const fn new(data: Vec<Pixel>, width: u16, height: u16) -> Self {
        Image {
            data,
            width,
            height,
        }
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub fn to_raster(&self) -> Vec<u8> {
        let pixels = self.get_pixels();

        let mut raster = Vec::new();

        for i in 0..self.height {
            for j in 0..(self.width + 7) / 8 {
                let mut byte = 0x00;
                for k in 0..8 {
                    let pixel = pixels
                        .get(i as usize)
                        .and_then(|line| line.get((j * 8 + k) as usize));
                    let pixel = pixel.unwrap_or(&DEFAULT_PIXEL);

                    if pixel.is_gray() {
                        let mask = 1 << (7 - k);
                        byte |= mask;
                    }
                }
                raster.push(byte);
            }
        }
        raster
    }

    pub fn get_pixels(&self) -> Vec<Vec<Pixel>> {
        let mut pixels = Vec::new();
        for i in 0..self.height {
            let mut line = Vec::new();
            for j in 0..self.width {
                let index = (self.width * i + j) << 2;
                line.push(self.data[index as usize]);
            }
            pixels.push(line);
        }
        pixels
    }
}

#[cfg(feature = "image")]
impl Into<Image> for ::image::DynamicImage {
    fn into(self) -> Image {
        // Convert the image to an RGBA image buffer
        let rgba_img = self.to_rgba8();

        // Get width and height of the image
        let width = rgba_img.width();
        let height = rgba_img.height();

        // Extract pixel data
        let mut pixels = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_img.get_pixel(x, y);
                let rgba = pixel.0;
                pixels.push(Pixel::new(rgba[0], rgba[1], rgba[2], rgba[3]));
            }
        }

        // Create the image
        Image::new(pixels, width as u16, height as u16)
    }
}
