use crate::error::Error;
use cluster::Cluster;
use cluster_object::ClusterObject;
use context_selector::ContextSelector;
use iced::widget::{column, container, text};
use iced::Command;
use iced::Length;
use iced::Settings;
use iced::Theme;
use iced::{Application, Element};
use messages::{ClusterMessage, Message};
use std::time;
use custom_widgets::toast::{Toast, self};

mod cluster;
mod cluster_object;
mod colours;
mod context_selector;
mod error;
mod kube_context;
mod kube_interface;
mod messages;
mod resource_type;
mod sizes;
mod utils;
mod workloads;
mod custom_widgets;

/// Based on the pokedex entry from the iced repo
pub fn main() -> iced::Result {
    WorkloadExplorer::run(Settings::default())
}

#[derive(Debug)]
struct WorkloadExplorer {
    cluster: Option<Cluster>,
    error: Option<Error>,
    context_selector: Option<ContextSelector>,
    toasts: Vec<Toast>,
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
                context_selector: None,
                toasts: vec![],
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

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ContextLoaded(Ok(context)) => {
                self.context_selector = None;
                self.cluster = Some(Cluster::new(context.clone(), None));

                Command::perform(
                    kube_interface::fetch_cluster_state(context.clone()),
                    |res| Message::ClusterMessage(ClusterMessage::WorkloadsLoaded(res)),
                )
            }
            Message::ClusterMessage(message) => match self.cluster {
                Some(..) => self
                    .cluster
                    .as_mut()
                    .expect("content from cluster received, so should be connect to cluster")
                    .update(message),
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
                println!("{}", error.get_message());
                Command::batch(vec![
                    Command::perform(utils::resolved(), |_ignored: ()| {
                        Message::AddToast("Unable to load given context. Please select a different context.".into())
                    }),
                    Command::perform(utils::resolved(), |_ignored| {
                        Message::ChangeContextRequested
                    }),
                ])
            }
            Message::ChangeContextRequested => {
                self.cluster = None;
                self.error = None;

                Command::perform(
                    kube_interface::get_all_contexts(),
                    Message::AllContextsLoaded,
                )
            }
            Message::AllContextsLoaded(Ok(all_contexts)) => {
                self.context_selector = Some(ContextSelector::new(all_contexts));

                Command::none()
            }
            Message::AllContextsLoaded(Err(error)) => {
                self.error = Some(error);

                Command::none()
            }
            Message::ContextSelectorMessage(message) => {
                self.context_selector.as_mut().unwrap().update(message)
            }
            Message::ContextSelected(kube_ctx_name) => Command::perform(
                kube_interface::load_named_context(kube_ctx_name),
                Message::ContextLoaded,
            ),
            Message::AddToast(message) => {
                self.toasts.push(Toast {
                    title: "Error".into(),
                    body: message,
                    status: toast::Status::Danger,
                });

                Command::none()
            }
            Message::CloseToast(index) => {
                self.toasts.remove(index);

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.error.is_some() {
            container(
                column![text(self.error.as_ref().unwrap().get_message())
                    .size(40)
                    .style(colours::get_red()),]
                .width(Length::Shrink),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        } else if self.cluster.is_some() {
            container(self.cluster.as_ref().unwrap().view())
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(sizes::P)
                .into()
        } else if self.context_selector.is_some() {
            self.context_selector.as_ref().unwrap().view()
        } else {
            container(text("loading..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        };

        toast::Manager::new(content, &self.toasts, Message::CloseToast)
            .timeout(toast::DEFAULT_TIMEOUT * 3)
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
