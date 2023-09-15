use crate::{
    cluster_object::ClusterObject, error::Error, kube_context::KubeContext, workloads::Workloads,
};

#[derive(Debug, Clone)]
pub enum Message {
    ContextLoaded(Result<KubeContext, Error>),
    ClusterMessage(ClusterMessage),
    ChangeContextRequested,
    AllContextsLoaded(Result<Vec<String>, Error>),
    ContextSelectorMessage(ContextSelectorMessage),
    CloseToast(usize),
    AddToast(String),
}

#[derive(Debug, Clone)]
pub enum ClusterMessage {
    WorkloadsLoaded(Result<Workloads, Error>),
    ChangeNamespaceRequested,
    NamespaceFieldChanged(String),
    NamespaceSelected(String),
    NamespaceChecked(Result<(), Error>),
    ReloadRequested,
    DeleteRequested(ClusterObject),
    Deleted(Result<ClusterObject, Error>),
}

impl Into<Message> for ClusterMessage {
    fn into(self) -> Message {
        Message::ClusterMessage(self)
    }
}

#[derive(Debug, Clone)]
pub enum ContextSelectorMessage {
    DropDownItemSelected(String),
    DropDownClosed,
    ContextSelected(String),
}

impl From<ContextSelectorMessage> for Message {
    fn from(value: ContextSelectorMessage) -> Self {
        Message::ContextSelectorMessage(value)
    }
}