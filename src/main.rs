use crate::error::Error;
use cluster::Cluster;
use cluster_object::ClusterObject;
use iced::widget::{column, container, text};
use iced::Command;
use iced::Length;
use iced::Settings;
use iced::Theme;
use iced::{Application, Element};
use messages::{ClusterMessage, Message};
use std::time;

mod cluster;
mod cluster_object;
mod colours;
mod error;
mod kube_context;
mod kube_interface;
mod messages;
mod resource_type;
mod sizes;
mod workloads;

/// Based on the pokedex entry from the iced repo
pub fn main() -> iced::Result {
    WorkloadExplorer::run(Settings::default())
}

#[derive(Debug)]
struct WorkloadExplorer {
    cluster: Option<Cluster>,
    error: Option<Error>,
}

impl Application for WorkloadExplorer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            WorkloadExplorer {
                cluster: None,
                error: None,
            },
            Command::perform(
                kube_interface::fetch_current_context(),
                Message::ContextLoaded,
            ),
        )
    }

    fn title(&self) -> String {
        "Workload Explorer".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ContextLoaded(Ok(context)) => {
                self.cluster = Some(Cluster::new(context.clone(), None));

                Command::perform(
                    kube_interface::fetch_cluster_state(context.clone()),
                    |res| Message::ClusterMessage(ClusterMessage::WorkloadsLoaded(res)),
                )
            }
            Message::ClusterMessage(message) => match self.cluster {
                Some(..) => self.cluster.as_mut().expect("content from cluster received, so should be connect to cluster").update(message),
                None => Command::none(),
            },
            Message::DeleteRequested(cluster_object) => {
                Command::perform(ClusterObject::delete(cluster_object), Message::Deleted)
            }
            Message::Deleted(result) => {
                match result {
                    Err(error) => {
                        self.error = Some(error);
                    }
                    Ok(..) => {}
                }

                Command::none()
            }
            Message::ContextLoaded(Err(error)) => {
                self.error = Some(error);

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Element<Message> = if self.error.is_some() {
            column![text(self.error.as_ref().unwrap().get_message())
                .size(40)
                .style(colours::get_red()),]
            .width(Length::Shrink)
            .into()
        } else if self.cluster.is_some() {
            return container(self.cluster.as_ref().unwrap().view())
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(sizes::P)
                .into();
        } else {
            text("set ctx :D").into()
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        match self.cluster {
            Some(..) => iced::time::every(time::Duration::from_secs(2))
                .map(|_instant| Message::ClusterMessage(ClusterMessage::ReloadRequested)),
            None => iced::Subscription::none(),
        }
    }
}
