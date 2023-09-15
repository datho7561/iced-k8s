use iced::{
    widget::button::{self},
    Theme, Vector,
};

use crate::{colours, sizes};

pub enum ButtonTheme {
    Primary,
    Secondary,
}

impl iced::widget::button::StyleSheet for ButtonTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let colour = match self {
            ButtonTheme::Primary => colours::get_blue(),
            ButtonTheme::Secondary => colours::get_lilac(),
        };

        button::Appearance {
            background: Some(colour.into()),
            border_radius: sizes::BORDER_RADIUS.into(),
            text_color: match self {
                ButtonTheme::Primary => colours::get_white().into(),
                ButtonTheme::Secondary => colours::get_black().into(),
            },
            ..Default::default()
        }
    }
}

pub fn as_button_theme(theme: ButtonTheme) -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(theme))
}
