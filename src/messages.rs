use crate::{
    cluster_object::ClusterObject, error::Error, kube_context::KubeContext, workloads::Workloads,
};

#[derive(Debug, Clone)]
pub enum Message {
    ContextLoaded(Result<KubeContext, Error>),
    ClusterMessage(ClusterMessage),
    DeleteRequested(ClusterObject),
    Deleted(Result<ClusterObject, Error>),
    ChangeContextRequested,
    AllContextsLoaded(Result<Vec<String>, Error>),
    ContextSelectorMessage(ContextSelectorMessage),
    ContextSelected(String),
    CloseToast(usize),
    AddToast(String),
}

#[derive(Debug, Clone)]
pub enum ClusterMessage {
    WorkloadsLoaded(Result<Workloads, Error>),
    ReloadRequested,
}

#[derive(Debug, Clone)]
pub enum ContextSelectorMessage {
    DropDownItemSelected(String),
    DropDownClosed,
}
