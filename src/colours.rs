use iced::Color;

/// TODO: replace
/// this is probably too harsh
pub fn get_red() -> Color {
    Color::from([1.0, 0.0, 0.0])
}

/// HERE IS A FUN COLOUR SCHEME
/// I USED COOLORS
/// <palette>
///   <color name="Lavender blush" hex="fae8eb" r="250" g="232" b="235" />
///   <color name="Tea rose (red)" hex="f6caca" r="246" g="202" b="202" />
///   <color name="Cordovan" hex="8e3e48" r="142" g="62" b="72" />
///   <color name="Lilac" hex="cd9fcc" r="205" g="159" b="204" />
///   <color name="Federal blue" hex="0a014f" r="10" g="1" b="79" />
/// </palette>

/// Lavendar blush
/// background and white text
pub fn get_white() -> Color {
    Color::from([250.0/255.0, 232.0/255.0, 235.0/255.0])
}

/// Federal blue
/// strong accent
pub fn get_blue() -> Color {
    Color::from([10.0/255.0, 1.0/255.0, 79.0/255.0])
}

/// Teal Rose
/// secondary text
pub fn get_grey() -> Color {
    desaturate(Color::from([142.0/255.0, 62.0/255.0, 72.0/255.0]), 0.4)
}

/// Lilac
/// Secondary accent
pub fn get_lilac() -> Color {
    Color::from([205.0/255.0, 159.0/255.0, 204.0/255.0])
}

/// TODO: replace, this is probably too harsh
/// black text
pub fn get_black() -> Color {
    Color::from([0.0, 0.0, 0.0])
}

/// desaturate the given colour.
/// probably a little wrong but works okay.
/// based on https://stackoverflow.com/questions/70966873/algorithm-to-desaturate-rgb-color
pub fn desaturate(colour: Color, amt: f32) -> Color {
    if amt < 0.0 || amt >= 1.0 {
        panic!("desaturation amount must be 0 < amt < 1");
    }
    let lum = 0.3 * colour.r + 0.6 * colour.g + 0.11 * colour.b;
    let inv = 1.0 - amt;
    // use lerp
    Color {
        r: colour.r * inv + lum * amt,
        g: colour.g * inv + lum * amt,
        b: colour.b * inv + lum * amt,
        a: colour.a,
    }
}
