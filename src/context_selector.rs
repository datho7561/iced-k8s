use crate::button_theme::as_button_theme;
use crate::button_theme::ButtonTheme;
use crate::colours;
use crate::container_theme::as_container_theme;
use crate::container_theme::ContainerTheme;
use crate::custom_widgets::circular_loading_spinner;
use crate::custom_widgets::circular_loading_spinner::Circular;
use crate::kube_interface;
use crate::messages::ContextSelectorMessage;
use crate::messages::Message;
use crate::sizes;
use crate::sizes::H2;
use iced::widget::container;
use iced::widget::horizontal_space;
use iced::widget::row;
use iced::widget::{
    button, column, combo_box as combo_box_fun,
    combo_box::{self},
    text,
};
use iced::Length;
use iced::Padding;
use iced::{Command, Element};

#[derive(Debug, Clone)]
pub struct ContextSelector {
    state: combo_box::State<String>,
    selection: Option<String>,
    loading: bool,
}

impl ContextSelector {
    pub fn new(contexts: Vec<String>) -> ContextSelector {
        ContextSelector {
            state: combo_box::State::new(contexts),
            selection: None,
            loading: false,
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
            ContextSelectorMessage::ContextSelected(kube_ctx_name) => {
                self.loading = true;
                Command::perform(
                    kube_interface::load_named_context(kube_ctx_name),
                    Message::ContextLoaded,
                )
            }
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
            Some(selection) => set_context_button
                .on_press(ContextSelectorMessage::ContextSelected(selection.clone()).into()),
            None => set_context_button,
        };

        let loading: Element<Message> = container(if self.loading {
            Into::<Element<Message>>::into(Circular::new())
        } else {
            text("").into()
        }).width(Length::Fill).align_x(iced::alignment::Horizontal::Center).into();

        container(
            column![
                text("Pick a new context to use")
                    .size(H2)
                    .style(colours::get_black()),
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
                row![
                    horizontal_space(Length::Fill),
                    set_context_button.style(as_button_theme(ButtonTheme::Primary))
                ],
                loading,
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
