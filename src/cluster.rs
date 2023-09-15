use iced::{
    advanced::widget::Operation,
    alignment,
    widget::{
        self, button, column, container, horizontal_space, row, text, text_input, vertical_rule,
    },
    Alignment, Background, Color, Command, Element, Length, Padding, Theme,
};

use crate::{
    button_theme::{as_button_theme, ButtonTheme},
    colours,
    container_theme::{as_container_theme, ContainerTheme},
    custom_widgets::circular_loading_spinner::{self, Circular},
    kube_context::KubeContext,
    kube_interface,
    messages::{ClusterMessage, Message},
    sizes, utils,
    workloads::Workloads,
};

#[derive(Debug, Clone)]
enum View {
    ListClusterItems,
    SetNamespace,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    context: KubeContext,
    workloads: Option<Workloads>,
    view: View,
    namespace_field_value: String,
}

impl Cluster {
    pub fn new(context: KubeContext, workloads: Option<Workloads>) -> Cluster {
        Cluster {
            context,
            workloads,
            view: View::ListClusterItems,
            namespace_field_value: String::from(""),
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self.view {
            View::ListClusterItems => {
                let header = container(
                    container(
                        row![
                            text("iced-k8s").size(sizes::H1).style(colours::get_white()),
                            vertical_rule(sizes::P),
                            self.context.view(),
                            horizontal_space(Length::Fill),
                            button(container(text("Change Namespace")).padding(Padding {
                                bottom: 0.0,
                                top: 0.0,
                                left: sizes::SEP,
                                right: sizes::SEP,
                            }))
                            .on_press(Message::ClusterMessage(
                                ClusterMessage::ChangeNamespaceRequested
                            ))
                            .style(as_button_theme(ButtonTheme::Secondary)),
                            button(container(text("Change Context")).padding(Padding {
                                bottom: 0.0,
                                top: 0.0,
                                left: sizes::SEP,
                                right: sizes::SEP,
                            }))
                            .on_press(Message::ChangeContextRequested)
                            .style(as_button_theme(ButtonTheme::Secondary)),
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
            View::SetNamespace => {
                let mut set_namespace_button =
                    button(container(text("Set Namespace")).padding(Padding {
                        top: 0.0,
                        bottom: 0.0,
                        left: sizes::SEP,
                        right: sizes::SEP,
                    }));
                set_namespace_button = if !self.namespace_field_value.is_empty() {
                    set_namespace_button.on_press(Message::ClusterMessage(
                        ClusterMessage::NamespaceSelected(self.namespace_field_value.clone()),
                    ))
                } else {
                    set_namespace_button
                };

                container(
                    column![
                        text("Pick a new namespace to use")
                            .size(sizes::H2)
                            .style(colours::get_black()),
                        text_input("New namespace", &self.namespace_field_value).on_input(
                            |value| Message::ClusterMessage(ClusterMessage::NamespaceFieldChanged(
                                value
                            ))
                        ),
                        row![
                            button(container(text("Change Context")).padding(Padding {
                                bottom: 0.0,
                                top: 0.0,
                                left: sizes::SEP,
                                right: sizes::SEP,
                            }))
                            .on_press(Message::ChangeContextRequested)
                            .style(as_button_theme(ButtonTheme::Primary)),
                            horizontal_space(Length::Fill),
                            set_namespace_button.style(as_button_theme(ButtonTheme::Primary))
                        ],
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
                            "Unable to load workloads for this namespace. Please select a different namespace or context".into(),
                        )
                    }),
                    Command::perform(utils::resolved(), |_ignored| {
                        ClusterMessage::ChangeNamespaceRequested.into()
                    }),
                ])
            }
            ClusterMessage::ReloadRequested => match self.view {
                View::ListClusterItems => Command::perform(
                    kube_interface::fetch_cluster_state(self.context.clone()),
                    |res| Message::ClusterMessage(ClusterMessage::WorkloadsLoaded(res)),
                ),
                View::SetNamespace => Command::none(),
            },
            ClusterMessage::DeleteRequested(cluster_object) => Command::perform(
                kube_interface::delete(self.context.clone(), cluster_object),
                |res| Message::ClusterMessage(ClusterMessage::Deleted(res)),
            ),
            ClusterMessage::Deleted(result) => match result {
                Err(_error) => Command::perform(utils::resolved(), |_ignored| {
                    Message::AddToast(String::from("Failed to delete resource"))
                }),
                Ok(..) => Command::none(),
            },
            ClusterMessage::ChangeNamespaceRequested => {
                self.view = View::SetNamespace;

                Command::none()
            }
            ClusterMessage::NamespaceFieldChanged(value) => {
                self.namespace_field_value = value;

                Command::none()
            }
            ClusterMessage::NamespaceSelected(new_namespace) => Command::perform(
                kube_interface::check_namespace_accessible(KubeContext::new(
                    self.context.get_config().clone(),
                    new_namespace,
                )),
                |res| ClusterMessage::NamespaceChecked(res).into(),
            ),
            ClusterMessage::NamespaceChecked(res) => match res {
                Ok(..) => {
                    self.context = KubeContext::new(
                        self.context.get_config().clone(),
                        self.namespace_field_value.clone(),
                    );
                    self.view = View::ListClusterItems;
                    self.namespace_field_value = String::from("");

                    Command::perform(utils::resolved(), |_ignored| {
                        ClusterMessage::ReloadRequested.into()
                    })
                }
                Err(error) => {
                    self.namespace_field_value = String::from("");
                    println!("{}", error.get_message());

                    Command::perform(utils::resolved(), |_ignored| {
                        Message::AddToast(String::from("The given namespace couldn't be accessed. Make sure it exists and you have permission to access it."))
                    })
                }
            },
        }
    }
}
