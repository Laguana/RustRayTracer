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

    pub fn dot_prod(&self, other: &Triple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn vec_add(&self, other: &Triple) -> Triple {
        return Triple::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }

    pub fn vec_sub(&self, other: &Triple) -> Triple {
        return Triple::new(self.x - other.x, self.y - other.y, self.z - other.z);
    }

    pub fn scale(&self, c: f32) -> Triple {
        return Triple::new(self.x * c, self.y * c, self.z * c);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Triple,
    pub direction: Triple,
}
