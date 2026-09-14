#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Triple {
    pub fn new(x: f32, y: f32, z: f32) -> Triple {
        return Triple { x, y, z };
    }

    pub fn unit_vector(&self) -> Triple {
        let magnitude = self.dot_prod(self).sqrt();
        return Triple::new(self.x / magnitude, self.y / magnitude, self.z / magnitude);
    }

    pub fn dot_prod<T: std::borrow::Borrow<Triple>>(&self, other: T) -> f32 {
        let other = other.borrow();
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Add for Triple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<&Triple> for Triple {
    type Output = Triple;

    fn add(self, rhs: &Triple) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Triple> for &Triple {
    type Output = Triple;

    fn add(self, rhs: Triple) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<&Triple> for &Triple {
    type Output = Triple;

    fn add(self, rhs: &Triple) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Triple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<&Triple> for Triple {
    type Output = Self;

    fn sub(self, rhs: &Triple) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Triple> for &Triple {
    type Output = Triple;

    fn sub(self, rhs: Triple) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Triple {
    type Output = Triple;

    fn mul(self, c: f32) -> Self::Output {
        return Triple::new(self.x * c, self.y * c, self.z * c);
    }
}

impl std::ops::Mul<f32> for &Triple {
    type Output = Triple;

    fn mul(self, c: f32) -> Self::Output {
        return Triple::new(self.x * c, self.y * c, self.z * c);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Triple,
    pub direction: Triple,
}
