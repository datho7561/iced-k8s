use crate::{workloads::Workloads, error::Error, cluster_object::ClusterObject};

#[derive(Debug, Clone)]
pub enum Message {
    WorkloadsLoaded(Result<Workloads, Error>),
    ReloadRequested,
    DeleteRequested(ClusterObject),
    Deleted(Result<ClusterObject, Error>),
}