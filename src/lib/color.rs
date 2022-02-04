pub type RGBA = (f32, f32, f32, f32);

pub fn lerp_color((ar, ag, ab, aa): RGBA, (br, bg, bb, ba): RGBA, t: f32) -> RGBA {
    let t = t.max(0.0).min(1.0);
    let ti = 1.0 - t;
    (
        ar * ti + br * t,
        ag * ti + bg * t,
        ab * ti + bb * t,
        aa * ti + ba * t,
    )
}
