use iced::{Theme, widget::container::{self}};

use crate::colours;

pub enum ContainerTheme {
    Light,
    Dark,
}

impl iced::widget::container::StyleSheet for ContainerTheme {
    type Style = Theme;

    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        container::Appearance {
            background: Some(match self {
                ContainerTheme::Dark => colours::get_blue().into(),
                ContainerTheme::Light => colours::get_white().into(),
            }),
            ..Default::default()
        }
    }
}

pub fn as_container_theme(theme: ContainerTheme) -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(
        theme,
    ))
}