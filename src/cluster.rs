use iced::{
    widget::{self, button, column, container, horizontal_space, row, text, vertical_rule},
    Alignment, Background, Color, Command, Element, Length, Padding, Theme,
};

use crate::{
    button_theme::{as_button_theme, ButtonTheme},
    colours,
    container_theme::{as_container_theme, ContainerTheme},
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
                    self.context.view(),
                    horizontal_space(Length::Fill),
                    button(container(text("Change Context")).padding(Padding {
                        bottom: 0.0,
                        top: 0.0,
                        left: sizes::SEP,
                        right: sizes::SEP,
                    }))
                    .on_press(Message::ChangeContextRequested)
                    .style(as_button_theme(ButtonTheme::Secondary))
                ]
                .width(Length::Fill)
                .spacing(sizes::SEP)
                .align_items(Alignment::Center),
            )
            .height(sizes::H1 + sizes::P * 2.0)
            .padding(Padding {
                bottom: sizes::SEP,
                top: sizes::SEP,
                left: 2.0 * sizes::SEP,
                right: 2.0 * sizes::SEP,
            }),
        )
        .style(as_container_theme(ContainerTheme::Dark));

        let workloads_content: Element<Message> = match &self.workloads {
            Some(workloads) => container(workloads.view())
                .padding(sizes::SEP)
                .style(as_container_theme(ContainerTheme::Light))
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
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
            .style(as_container_theme(ContainerTheme::Light))
            .into(),
        };

        column![header, workloads_content]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
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
