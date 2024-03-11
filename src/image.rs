use crate::pixel::{Pixel, DEFAULT_PIXEL};

pub struct Image<'a> {
    data: &'a [Pixel],
    width: u16,
    height: u16,
}

impl<'a> Image<'a> {
    pub const fn new(data: &'a [Pixel], width: u16, height: u16) -> Self {
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
