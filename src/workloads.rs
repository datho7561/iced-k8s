use crate::cluster_object::ClusterObject;
use crate::colours;
use crate::error::Error;
use crate::resource_type::ResourceType;
use crate::sizes;
use crate::Message;
use iced::widget::{button, column, container, horizontal_space, row, text, vertical_rule};
use iced::{Alignment, Element, Length, Padding};
use k8s_openapi::api::apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet};
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use kube::{Api, Client, Config};

#[derive(Debug, Clone)]
pub struct Workloads {
    cluster_url: String,
    namespace: String,
    cluster_objects: Vec<ClusterObject>,
}

impl Workloads {
    pub fn view(&self) -> Element<Message> {
        let workload_elts: Vec<Element<Message>> = self
            .cluster_objects
            .iter()
            .map(|cluster_object| cluster_object.view())
            .collect();

        let deployment_table: Element<Message> = column(workload_elts).spacing(sizes::SEP).into();

        let header = container(
            row![
                text("iced-k8s").size(sizes::H1),
                vertical_rule(sizes::P),
                horizontal_space(sizes::SEP),
                text("Cluster:").style(colours::get_grey()),
                text(&self.cluster_url),
                horizontal_space(sizes::SEP),
                text("Namespace:").style(colours::get_grey()),
                text(&self.namespace),
                horizontal_space(sizes::SEP),
                button(text("Reload")).on_press(Message::ReloadRequested)
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

        column![header, deployment_table,].into()
    }

    pub async fn fetch_cluster_state() -> Result<Workloads, Error> {
        // Infer the runtime environment and try to create a Kubernetes Client
        let client = Client::try_default().await.map_err(Error::from_k8s)?;

        let mut cluster_objects: Vec<ClusterObject> = vec![];

        let deployments: Api<Deployment> = Api::default_namespaced(client.clone());
        cluster_objects.extend(
            deployments
                .list(&ListParams::default())
                .await
                .map_err(Error::from_k8s)?
                .items
                .iter()
                .map(|deployment| {
                    let details = if deployment.status != None && deployment.spec != None {
                        let available = deployment
                            .status
                            .clone()
                            .unwrap()
                            .available_replicas
                            .unwrap_or(0);
                        let requested = deployment.spec.clone().unwrap().replicas.unwrap_or(0);
                        Some(format!("{}/{}", available, requested))
                    } else {
                        None
                    };
                    ClusterObject::new(
                        deployment.metadata.name.as_ref().unwrap().to_owned(),
                        ResourceType::Deployment,
                        details,
                    )
                }),
        );
        let daemonsets: Api<DaemonSet> = Api::default_namespaced(client.clone());
        cluster_objects.extend(
            daemonsets
                .list(&ListParams::default())
                .await
                .map_err(Error::from_k8s)?
                .items
                .iter()
                .map(|daemonset| {
                    ClusterObject::new(
                        daemonset.metadata.name.as_ref().unwrap().to_owned(),
                        ResourceType::DaemonSet,
                        None,
                    )
                }),
        );
        let replicasets: Api<ReplicaSet> = Api::default_namespaced(client.clone());
        cluster_objects.extend(
            replicasets
                .list(&ListParams::default())
                .await
                .map_err(Error::from_k8s)?
                .items
                .iter()
                .map(|replicaset| {
                    let details = if replicaset.status != None && replicaset.spec != None {
                        let available = replicaset
                            .status
                            .clone()
                            .unwrap()
                            .available_replicas
                            .unwrap_or(0);
                        let requested = replicaset.spec.clone().unwrap().replicas.unwrap_or(0);
                        Some(format!("{}/{}", available, requested))
                    } else {
                        None
                    };
                    ClusterObject::new(
                        replicaset.metadata.name.as_ref().unwrap().to_owned(),
                        ResourceType::ReplicaSet,
                        details,
                    )
                }),
        );
        let statefulsets: Api<StatefulSet> = Api::default_namespaced(client.clone());
        cluster_objects.extend(
            statefulsets
                .list(&ListParams::default())
                .await
                .map_err(Error::from_k8s)?
                .items
                .iter()
                .map(|statefulset| {
                    ClusterObject::new(
                        statefulset.metadata.name.as_ref().unwrap().to_owned(),
                        ResourceType::StatefulSet,
                        None,
                    )
                }),
        );
        let pods: Api<Pod> = Api::default_namespaced(client.clone());
        cluster_objects.extend(
            pods.list(&ListParams::default())
                .await
                .map_err(Error::from_k8s)?
                .items
                .iter()
                .map(|pod| {
                    let details = match &pod.status {
                        Some(status) => status.phase.clone(),
                        None => None,
                    };
                    ClusterObject::new(
                        pod.metadata.name.as_ref().unwrap().to_owned(),
                        ResourceType::Pod,
                        details,
                    )
                }),
        );

        let config = Config::infer().await.map_err(Error::from_k8s_config)?;

        Ok(Workloads {
            cluster_url: config.cluster_url.to_string(),
            namespace: config.default_namespace.clone(),
            cluster_objects,
        })
    }
}
