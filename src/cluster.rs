use iced::{
    gradient::Linear,
    widget::{self, button, column, container, horizontal_space, row, text, vertical_rule},
    Alignment, Background, Color, Command, Element, Length, Padding, Theme,
};

use crate::{
    colours,
    custom_widgets::circular_loading_spinner,
    kube_context::KubeContext,
    kube_interface,
    messages::{ClusterMessage, Message},
    sizes, utils,
    workloads::Workloads,
};

#[derive(Debug, Clone)]
pub struct Cluster {
    context: KubeContext,
    workloads: Option<Workloads>,
}

enum Dummy {
    Value,
}

impl iced::widget::container::StyleSheet for Dummy {
    type Style = Theme;

    fn appearance(&self, theme: &Theme) -> container::Appearance {
        container::Appearance {
            background: Some(colours::get_blue().into()),
            ..Default::default()
        }
    }
}

impl Cluster {
    pub fn new(context: KubeContext, workloads: Option<Workloads>) -> Cluster {
        Cluster { context, workloads }
    }

    pub fn view(&self) -> Element<Message> {
        let header = container(
            container(
                row![
                    text("iced-k8s").size(sizes::H1).style(colours::get_white()),
                    vertical_rule(sizes::P),
                    horizontal_space(sizes::SEP),
                    self.context.view(),
                    horizontal_space(Length::Fill),
                    button(text("Change Context")).on_press(Message::ChangeContextRequested)
                ]
                .width(Length::Fill)
                .spacing(sizes::SEP)
                .align_items(Alignment::Center),
            )
            .height(sizes::H1 + sizes::P * 2.0)
            .padding(Padding {
                bottom: sizes::P,
                top: 0.0,
                left: 0.0,
                right: 0.0,
            })
            .padding(sizes::SEP),
        )
        .style(iced::theme::Container::Custom(Box::new(Dummy::Value)));

        let workloads_content: Element<Message> = match &self.workloads {
            Some(workloads) => container(workloads.view()).padding(sizes::SEP).into(),
            None => container(
                row![
                    circular_loading_spinner::Circular::new(),
                    horizontal_space(sizes::SEP),
                    text("Loading workloads...")
                ]
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(sizes::SEP)
            .center_x()
            .center_y()
            .into(),
        };

        column![header, workloads_content].width(Length::Fill).into()
    }

    pub fn update(&mut self, message: ClusterMessage) -> iced::Command<Message> {
        match message {
            ClusterMessage::WorkloadsLoaded(Ok(workloads)) => {
                self.workloads = Some(workloads);

                Command::none()
            }
            ClusterMessage::WorkloadsLoaded(Err(error)) => {
                self.workloads = None;
                println!("{}", error.get_message());

                Command::batch(vec![
                    Command::perform(utils::resolved(), move |_ignored| {
                        Message::AddToast(
                            "Unable to load workloads. Make sure the cluster is accessible.".into(),
                        )
                    }),
                    Command::perform(utils::resolved(), |_ignored| {
                        Message::ChangeContextRequested
                    }),
                ])
            }
            ClusterMessage::ReloadRequested => Command::perform(
                kube_interface::fetch_cluster_state(self.context.clone()),
                |res| Message::ClusterMessage(ClusterMessage::WorkloadsLoaded(res)),
            ),
        }
    }
}
