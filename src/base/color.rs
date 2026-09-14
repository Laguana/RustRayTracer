#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<(f32, f32, f32, f32)> for RGBA {
    fn from((r,g,b,a): (f32, f32, f32, f32)) -> RGBA {
        return RGBA {
            r, g, b, a
        }
    }
}

impl From<RGBA> for sdl3::pixels::Color {
    fn from(c: RGBA) -> sdl3::pixels::Color {
        return sdl3::pixels::Color::RGBA(
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
            (c.a * 255.0) as u8)
    }
}

impl std::ops::Add<RGBA> for RGBA {
    type Output = RGBA;

    fn add(self, rhs: RGBA) -> Self::Output {
        (self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a).into()
    }
}

impl std::ops::Mul<RGBA> for RGBA {
    type Output = RGBA;

    fn mul(self, rhs: RGBA) -> Self::Output {
        (self.r * rhs.r, self.g * rhs.g, self.b * rhs.b, self.a * rhs.a).into()
    }
}

impl std::ops::Mul<f32> for RGBA {
    type Output = RGBA;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.r * rhs, self.g * rhs, self.b * rhs, self.a * rhs).into()
    }
}