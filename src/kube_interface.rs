use k8s_openapi::api::{
    apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet},
    core::v1::Pod,
};
use kube::{api::ListParams, Api, Client, Config};

use crate::{
    cluster_object::ClusterObject, error::Error, resource_type::ResourceType, workloads::Workloads, kube_context::KubeContext,
};

pub async fn fetch_current_context() -> Result<KubeContext, Error> {
    let config = Config::infer().await.map_err(Error::from_k8s_config)?;
    Ok(KubeContext::new(config))
}

pub async fn fetch_cluster_state(context: KubeContext) -> Result<Workloads, Error> {
    let client = Client::try_from(context.get_config().to_owned()).map_err(Error::from_k8s)?;

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

    Ok(Workloads::new(
        cluster_objects,
    ))
}
