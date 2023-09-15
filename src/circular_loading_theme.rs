use iced::{
    widget::button::{self},
    Theme, Vector,
};

use crate::{colours::{self, get_white}, sizes};
use crate::custom_widgets::circular_loading_spinner;

pub enum CircularLoadingTheme {
    Primary,
    Secondary,
}

impl circular_loading_spinner::StyleSheet for CircularLoadingTheme {
    type Style = Theme;

    fn appearance(&self, _style: &Theme) -> circular_loading_spinner::Appearance {
        let colour = match self {
            CircularLoadingTheme::Primary => colours::get_blue(),
            CircularLoadingTheme::Secondary => colours::get_lilac(),
        };
        circular_loading_spinner::Appearance {
            bar_color: colour,
            track_color: colours::get_white(),
            ..Default::default()
        }
    }
}

pub fn as_circular_theme(theme: CircularLoadingTheme) -> circular_loading_spinner::CircularStyle {
    circular_loading_spinner::CircularStyle::Custom(Box::new(theme))
}
