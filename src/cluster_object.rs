use iced::{
    widget::{button, row, text},
    Alignment, Element,
};
use k8s_openapi::api::{
    apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet},
    core::v1::Pod,
};
use kube::{api::DeleteParams, Api, Client};

use crate::{colours, error::Error, resource_type::ResourceType, sizes, Message};

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
            text(self.name.to_owned()).size(sizes::P).width(400),
            text(self.r#type.to_owned())
                .size(sizes::P)
                .style(colours::get_grey())
                .width(100),
            text(self.details.clone().unwrap_or(String::from("")))
                .style(colours::get_grey())
                .width(100),
            button(text("Delete"))
                .style(iced::theme::Button::Destructive)
                .on_press(Message::DeleteRequested(self.to_owned()))
        ]
        .spacing(sizes::P * 2.0)
        .align_items(Alignment::Center)
        .into()
    }

    pub async fn delete(cluster_object: ClusterObject) -> Result<ClusterObject, Error> {
        let client = Client::try_default().await?;
        client.default_namespace();

        match cluster_object.r#type {
            ResourceType::Pod => {
                let api: Api<Pod> = Api::default_namespaced(client);
                let delete_params = DeleteParams::default();
                let _ = api
                    .delete(cluster_object.name.as_str(), &delete_params)
                    .await?;
            }
            ResourceType::DaemonSet => {
                let api: Api<DaemonSet> = Api::default_namespaced(client);
                let delete_params = DeleteParams::default();
                let _ = api
                    .delete(cluster_object.name.as_str(), &delete_params)
                    .await?;
            }
            ResourceType::Deployment => {
                let api: Api<Deployment> = Api::default_namespaced(client);
                let delete_params = DeleteParams::default();
                let _ = api
                    .delete(cluster_object.name.as_str(), &delete_params)
                    .await?;
            }
            ResourceType::ReplicaSet => {
                let api: Api<ReplicaSet> = Api::default_namespaced(client);
                let delete_params = DeleteParams::default();
                let _ = api
                    .delete(cluster_object.name.as_str(), &delete_params)
                    .await?;
            }
            ResourceType::StatefulSet => {
                let api: Api<StatefulSet> = Api::default_namespaced(client);
                let delete_params = DeleteParams::default();
                let _ = api
                    .delete(cluster_object.name.as_str(), &delete_params)
                    .await?;
            }
        };
        Ok(cluster_object)
    }
}
