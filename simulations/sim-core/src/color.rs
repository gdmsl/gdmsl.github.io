/// RGBA color.
pub type Rgba = [u8; 4];

/// Electric blue color ramp: black → deep blue → electric blue → white.
/// Input `t` in [0, 1].
pub fn electric_blue(t: f64) -> Rgba {
    let t = t.clamp(0.0, 1.0);

    if t < 0.2 {
        // Black → deep navy
        let s = t / 0.2;
        [
            lerp_u8(2, 15, s),
            lerp_u8(6, 23, s),
            lerp_u8(23, 42, s),
            255,
        ]
    } else if t < 0.5 {
        // Deep navy → electric blue
        let s = (t - 0.2) / 0.3;
        [
            lerp_u8(15, 59, s),
            lerp_u8(23, 130, s),
            lerp_u8(42, 246, s),
            255,
        ]
    } else if t < 0.8 {
        // Electric blue → bright blue-white
        let s = (t - 0.5) / 0.3;
        [
            lerp_u8(59, 147, s),
            lerp_u8(130, 197, s),
            lerp_u8(246, 253, s),
            255,
        ]
    } else {
        // Bright → white
        let s = (t - 0.8) / 0.2;
        [
            lerp_u8(147, 248, s),
            lerp_u8(197, 250, s),
            lerp_u8(253, 252, s),
            255,
        ]
    }
}

/// Teal accent ramp for phase information.
pub fn teal_phase(t: f64) -> Rgba {
    let t = t.clamp(0.0, 1.0);

    if t < 0.3 {
        let s = t / 0.3;
        [
            lerp_u8(2, 4, s),
            lerp_u8(6, 47, s),
            lerp_u8(23, 46, s),
            255,
        ]
    } else if t < 0.6 {
        let s = (t - 0.3) / 0.3;
        [
            lerp_u8(4, 20, s),
            lerp_u8(47, 184, s),
            lerp_u8(46, 166, s),
            255,
        ]
    } else {
        let s = (t - 0.6) / 0.4;
        [
            lerp_u8(20, 153, s),
            lerp_u8(184, 246, s),
            lerp_u8(166, 228, s),
            255,
        ]
    }
}

/// Background color matching the site's dark theme.
pub const BG_DARK: Rgba = [2, 6, 23, 255];

/// Blend a simulation color with alpha over the background.
pub fn blend_over_bg(fg: Rgba, alpha: f64) -> Rgba {
    let a = alpha.clamp(0.0, 1.0);
    [
        lerp_u8(BG_DARK[0], fg[0], a),
        lerp_u8(BG_DARK[1], fg[1], a),
        lerp_u8(BG_DARK[2], fg[2], a),
        255,
    ]
}

fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    let v = a as f64 + (b as f64 - a as f64) * t;
    v.round().clamp(0.0, 255.0) as u8
}
