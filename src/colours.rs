use iced::Color;

pub fn get_grey() -> Color {
    Color::from([0.5, 0.5, 0.5])
}

pub fn get_black() -> Color {
    Color::from([0.0, 0.0, 0.0])
}


pub fn get_red() -> Color {
    Color::from([1.0, 0.0, 0.0])
}

/// desaturate the given colour.
/// probably a little wrong but works okay.
/// based on https://stackoverflow.com/questions/70966873/algorithm-to-desaturate-rgb-color
pub fn desaturate(colour: Color, amt: f32) -> Color {
    if amt < 0.0 || amt >= 1.0 {
        panic!("desaturation amount must be 0 < amt < 1");
    }
    let lum = 0.3*colour.r + 0.6*colour.g + 0.11*colour.b;
    let inv = 1.0 - amt;
    // use lerp
    Color {
        r: colour.r * inv + lum * amt,
        g: colour.g * inv + lum * amt,
        b: colour.b * inv + lum * amt,
        a: colour.a,
    }
}