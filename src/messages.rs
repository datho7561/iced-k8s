use crate::{
    cluster_object::ClusterObject, error::Error, kube_context::KubeContext, workloads::Workloads,
};

#[derive(Debug, Clone)]
pub enum Message {
    ContextLoaded(Result<KubeContext, Error>),
    ClusterMessage(ClusterMessage),
    DeleteRequested(ClusterObject),
    Deleted(Result<ClusterObject, Error>),
}

#[derive(Debug, Clone)]
pub enum ClusterMessage {
    WorkloadsLoaded(Result<Workloads, Error>),
    ReloadRequested
}