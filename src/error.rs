use kube::{config::InferConfigError, Error as KubeError};

#[derive(Debug, Clone)]
pub enum Error {
    KubernetesClientError(String),
}

impl Error {
    pub fn from_k8s(kube_error: KubeError) -> Error {
        Error::KubernetesClientError(kube_error.to_string())
    }

    pub fn from_k8s_config(kube_error: InferConfigError) -> Error {
        Error::KubernetesClientError(kube_error.to_string())
    }
}
