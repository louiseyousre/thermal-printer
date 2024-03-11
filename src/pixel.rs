use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel { r, g, b, a }
    }

    pub const fn is_gray(&self) -> bool {
        self.a > 0 && ((self.r + self.g + self.b) / 3) < 230
    }

    pub const fn red(&self) -> u8 {
        self.r
    }

    pub const fn green(&self) -> u8 {
        self.g
    }

    pub const fn blue(&self) -> u8 {
        self.b
    }

    pub const fn alpha(&self) -> u8 {
        self.a
    }
}

pub const DEFAULT_PIXEL: Pixel = Pixel::new(0, 0, 0, 0);

impl Default for Pixel {
    fn default() -> Self {
        DEFAULT_PIXEL
    }
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, other: Pixel) -> Pixel {
        Pixel {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, other: Pixel) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
        self.a += other.a;
    }
}

impl Sub for Pixel {
    type Output = Pixel;

    fn sub(self, other: Pixel) -> Pixel {
        Pixel {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

impl SubAssign for Pixel {
    fn sub_assign(&mut self, other: Pixel) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
        self.a -= other.a;
    }
}

impl Mul<u8> for Pixel {
    type Output = Pixel;

    fn mul(self, rhs: u8) -> Pixel {
        Pixel {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Div<u8> for Pixel {
    type Output = Pixel;

    fn div(self, rhs: u8) -> Pixel {
        Pixel {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}
