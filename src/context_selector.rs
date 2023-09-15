use crate::button_theme::ButtonTheme;
use crate::button_theme::as_button_theme;
use crate::colours;
use crate::container_theme::ContainerTheme;
use crate::container_theme::as_container_theme;
use crate::messages::ContextSelectorMessage;
use crate::messages::Message;
use crate::sizes;
use crate::sizes::H2;
use iced::Padding;
use iced::widget::container;
use iced::widget::horizontal_space;
use iced::widget::row;
use iced::widget::{
    button, column, combo_box as combo_box_fun,
    combo_box::{self},
    text,
};
use iced::Length;
use iced::{Command, Element};

#[derive(Debug, Clone)]
pub struct ContextSelector {
    state: combo_box::State<String>,
    selection: Option<String>,
}

impl ContextSelector {
    pub fn new(contexts: Vec<String>) -> ContextSelector {
        ContextSelector {
            state: combo_box::State::new(contexts),
            selection: None,
        }
    }

    pub fn update(&mut self, message: ContextSelectorMessage) -> Command<Message> {
        match message {
            ContextSelectorMessage::DropDownItemSelected(kube_ctx) => {
                self.selection = Some(kube_ctx);
                self.state.unfocus();

                Command::none()
            }
            ContextSelectorMessage::DropDownClosed => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut set_context_button = button(container(text("Set Context")).padding(Padding {
            bottom: 0.0,
            top: 0.0,
            left: sizes::SEP,
            right: sizes::SEP,
        }));
        set_context_button = match self.selection.as_ref() {
            Some(selection) => {
                set_context_button.on_press(Message::ContextSelected(selection.clone()))
            }
            None => set_context_button,
        };

        container(
            column![
                text("Pick a new context to use").size(H2).style(colours::get_black()),
                combo_box_fun(
                    &self.state,
                    "New context to view",
                    self.selection.as_ref(),
                    |selection| Message::ContextSelectorMessage(
                        ContextSelectorMessage::DropDownItemSelected(selection)
                    )
                )
                .on_close(Message::ContextSelectorMessage(
                    ContextSelectorMessage::DropDownClosed
                )),
                row![horizontal_space(Length::Fill), set_context_button.style(as_button_theme(ButtonTheme::Primary))],
            ]
            .max_width(400)
            .spacing(sizes::SEP),
        )
        .style(as_container_theme(ContainerTheme::Light))
        .padding(sizes::SEP)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into()
    }
}
