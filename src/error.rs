use kube::{config::InferConfigError, Error as KubeError};

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn from_k8s(kube_error: KubeError) -> Error {
        Error { message: kube_error.to_string() }
    }

    pub fn from_k8s_config(kube_error: InferConfigError) -> Error {
        Error { message: kube_error.to_string() }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
