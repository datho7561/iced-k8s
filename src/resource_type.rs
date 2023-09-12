#[derive(Debug, Clone)]
pub enum ResourceType {
    Pod,
    Deployment,
    ReplicaSet,
    StatefulSet,
    DaemonSet,
}

impl ToString for ResourceType {
    fn to_string(&self) -> String {
        match &self {
            Self::Pod => String::from("Pod"),
            Self::Deployment => String::from("Deployment"),
            Self::ReplicaSet => String::from("ReplicaSet"),
            Self::DaemonSet => String::from("DaemonSet"),
            Self::StatefulSet => String::from("StatefulSet"),
        }
    }
}
