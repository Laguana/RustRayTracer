use crate::base::ray::Triple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    r: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl Quaternion {
    pub fn norm(&self) -> f32 {
        let Quaternion {r: a, i: b, j: c, k: d} = self;
        (a*a + b*b + c*c + d*d).sqrt()
    }

    pub fn unit(&self) -> Quaternion {
        self / self.norm()
    }

    pub fn conjugate(&self) -> Quaternion {
        let Quaternion {r: a, i: b, j: c, k: d} = self;
        Quaternion { r: *a, i: -b, j: -c, k: -d}
    }

    pub fn inverse(&self) -> Quaternion {
        let norm = self.norm();
        self.conjugate() / (norm * norm)
    }
}

pub fn rotate(point: &Triple, axis: &Triple, radians: f32) -> Triple {
    let Triple{ x, y, z } = axis.unit_vector();
    // A rotation around an axis (x,y,z) by angle phi
    // corresponds to R a R^-1 where R = {r: cos(phi/2) .. }, the non-r parts of R are a vector pointing in the x y z direction
    // and a is { r: 0 .. } with the non-real parts being the 3d vector to rotate
    // To construct R given phi and axis, we can compute cos(phi/2) and sin(phi/2), and the result is {r: cos(phi/2) , i: 0, j: 0, k: 0} + sin(phi/2) * {r: 0, i: ax, j: ay, k: az}.unit
    let cos = (radians/2.0).cos();
    let sin = (radians/2.0).sin();
    let rotation = Quaternion { r: cos, i: sin * x, j: sin * y, k: sin * z };
    let Triple{ x: px, y: py, z: pz} = point;
    let subject = Quaternion { r: 0.0, i: *px, j: *py, k: *pz};
    let Quaternion{ i: rx, j: ry, k: rz,  .. } = rotation * subject * (rotation.conjugate()); 
    Triple{ x: rx, y: ry, z: rz }
}

impl std::ops::Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // ijk = -1  -ij = -k  ij = k
        //           -jk = -i  jk = i
        //           -ij = -k  j  = -ik
        // (a + bi + cj + dk) * (e + fi + gj + hk)
        // = ae + afi + agj + ahk + bei + -bf + bgij + bhik + cej + cfji - cg + chjk + dek + dfki + dgkj - dh
        // = (ae - bf - cg - dh) + (af + be + ch - dg)i + (ag + ce - bh + df)j + (ah + de + bg - cf)k
        let Self { r: a, i: b, j: c, k: d} = self;
        let Self { r: e, i: f, j: g, k: h} = rhs;
        Self::Output {
            r: a * e - b * f - c * g - d * h,
            i: a * f + b * e + c * h - d * g,
            j: a * g + c * e - b * h + d * f,
            k: a * h + d * e + b * g - c * f,
        }
    }
}

impl std::ops::Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let Self { r: a, i: b, j: c, k: d} = self;

        Self::Output {
            r: a * rhs,
            i: b * rhs,
            j: c * rhs,
            k: d * rhs,
        }
    }
}

impl std::ops::Mul<f32> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self::Output {
        let Quaternion { r: a, i: b, j: c, k: d} = *self;

        Self::Output {
            r: a * rhs,
            i: b * rhs,
            j: c * rhs,
            k: d * rhs,
        }
    }
}

impl std::ops::Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Division by 0")
        }
        let Self { r: a, i: b, j: c, k: d} = self;

        Self::Output {
            r: a / rhs,
            i: b / rhs,
            j: c / rhs,
            k: d / rhs,
        }
    }
}


impl std::ops::Div<f32> for &Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Division by 0")
        }
        let Quaternion { r: a, i: b, j: c, k: d} = *self;

        Self::Output {
            r: a / rhs,
            i: b / rhs,
            j: c / rhs,
            k: d / rhs,
        }
    }
}