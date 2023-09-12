use crate::error::Error;
use crate::workloads::Workloads;
use cluster_object::ClusterObject;
use iced::widget::{column, container, text};
use iced::Application;
use iced::Command;
use iced::Length;
use iced::Settings;
use iced::Theme;
use messages::Message;
use std::time;

mod cluster_object;
mod colours;
mod error;
mod messages;
mod resource_type;
mod sizes;
mod workloads;

// Based on the pokedex entry from the iced repo
//
pub fn main() -> iced::Result {
    WorkloadExplorer::run(Settings::default())
}

#[derive(Debug)]
enum WorkloadExplorer {
    Fetching { workloads: Option<Workloads> },
    Fetched { workloads: Workloads },
    Errored { message: String },
}

impl Application for WorkloadExplorer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            WorkloadExplorer::Fetching {
                workloads: Option::None,
            },
            Command::perform(Workloads::fetch_cluster_state(), Message::WorkloadsLoaded),
        )
    }

    fn title(&self) -> String {
        "Workload Explorer".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::WorkloadsLoaded(Ok(workloads)) => {
                *self = WorkloadExplorer::Fetched { workloads };

                Command::none()
            }
            Message::WorkloadsLoaded(Err(error)) => {
                *self = WorkloadExplorer::Errored {
                    message: match error {
                        Error::KubernetesClientError(message) => message,
                    },
                };

                Command::none()
            }
            Message::ReloadRequested => {
                *self = WorkloadExplorer::Fetching {
                    workloads: match self {
                        Self::Fetched { workloads } => Some(workloads.clone()),
                        Self::Fetching { workloads } => workloads.clone(),
                        Self::Errored { .. } => None,
                    },
                };

                Command::perform(Workloads::fetch_cluster_state(), Message::WorkloadsLoaded)
            }
            Message::DeleteRequested(cluster_object) => {
                Command::perform(ClusterObject::delete(cluster_object), Message::Deleted)
            }
            Message::Deleted(result) => {
                match result {
                    Err(error) => {
                        *self = WorkloadExplorer::Errored {
                            message: match error {
                                Error::KubernetesClientError(message) => message,
                            },
                        };
                    }
                    Ok(..) => {}
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = match self {
            WorkloadExplorer::Fetching { workloads } => match workloads {
                Some(existing_workloads) => {
                    return container(existing_workloads.view())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(sizes::P)
                        .into()
                }
                None => column![text("Fetching workloads...").size(40),].width(Length::Shrink),
            },
            WorkloadExplorer::Errored { message } => {
                column![text(message).size(40).style(colours::get_red()),].width(Length::Shrink)
            }
            WorkloadExplorer::Fetched { workloads } => {
                return container(workloads.view())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(sizes::P)
                    .into()
            }
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        match *self {
            WorkloadExplorer::Errored { .. } => iced::Subscription::none(),
            WorkloadExplorer::Fetching { .. } => iced::Subscription::none(),
            WorkloadExplorer::Fetched { .. } => iced::time::every(time::Duration::from_secs(2))
                .map(|_instant| Message::ReloadRequested),
        }
    }
}
