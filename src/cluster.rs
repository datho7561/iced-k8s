use iced::{
    widget::{button, column, container, horizontal_space, row, text, vertical_rule},
    Alignment, Command, Element, Length, Padding,
};

use crate::{
    kube_context::KubeContext,
    kube_interface,
    messages::{ClusterMessage, Message},
    sizes,
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
            row![
                text("iced-k8s").size(sizes::H1),
                vertical_rule(sizes::P),
                horizontal_space(sizes::SEP),
                self.context.view(),
                horizontal_space(sizes::SEP),
                button(text("Reload"))
                    .on_press(Message::ClusterMessage(ClusterMessage::ReloadRequested))
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
        });

        let workloads_content: Element<Message> = match &self.workloads {
            Some(workloads) => workloads.view(),
            None => {
                container(text("Unable to load workloads"))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        };

        column![header, workloads_content].into()
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

                Command::none()
            }
            ClusterMessage::ReloadRequested => {
                Command::perform(kube_interface::fetch_cluster_state(self.context.clone()), |res| {
                    Message::ClusterMessage(ClusterMessage::WorkloadsLoaded(res))
                })
            }
        }
    }
}
