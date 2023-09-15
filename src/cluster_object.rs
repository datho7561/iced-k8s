use iced::{
    widget::{button, container, horizontal_space, row, text},
    Alignment, Element, Length, Padding,
};
use k8s_openapi::api::{
    apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet},
    core::v1::Pod,
};
use kube::{api::DeleteParams, Api, Client};

use crate::{
    colours, error::Error, kube_context::KubeContext, messages::ClusterMessage,
    resource_type::ResourceType, sizes, Message,
};

#[derive(Debug, Clone)]
pub struct ClusterObject {
    pub name: String,
    pub r#type: ResourceType,
    pub details: Option<String>,
    pub children: Vec<ClusterObject>,
}

impl ClusterObject {
    pub fn new(name: String, r#type: ResourceType, details: Option<String>) -> ClusterObject {
        ClusterObject {
            name,
            r#type,
            children: vec![],
            details,
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            text(self.name.to_owned())
                .size(sizes::P)
                .width(400)
                .style(colours::get_black()),
            text(self.r#type.to_owned())
                .size(sizes::P)
                .style(colours::get_grey())
                .width(100),
            text(self.details.clone().unwrap_or(String::from("")))
                .style(colours::get_grey())
                .width(100),
            horizontal_space(Length::Fill),
            button(
                container(text("Delete").style(colours::get_white())).padding(Padding {
                    bottom: 0.0,
                    top: 0.0,
                    left: sizes::SEP,
                    right: sizes::SEP,
                })
            )
            .style(iced::theme::Button::Destructive)
            .on_press(Message::ClusterMessage(ClusterMessage::DeleteRequested(
                self.to_owned()
            )))
        ]
        .spacing(sizes::P * 2.0)
        .align_items(Alignment::Center)
        .into()
    }
}
