
pub type Triple = (f32, f32, f32);

pub fn sq_magnitude((x,y,z): &Triple) -> f32 {
    x*x+y*y+z*z
}

pub fn unit_vector(v: &Triple) -> Triple {
    let magnitude = sq_magnitude(v).sqrt();
    let (x,y,z) = v;
    (x/magnitude, y/magnitude, z/magnitude)
}

pub fn dot_prod((ax,ay,az): &Triple, (bx,by,bz): &Triple) -> f32 {
    ax*bx + ay*by + az*bz
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Triple,
    direction: Triple,
}