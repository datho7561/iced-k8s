use kube::{
    config::{InferConfigError, KubeconfigError},
    Error as KubeError,
};

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl From<KubeError> for Error {
    fn from(kube_error: KubeError) -> Self {
        Error {
            message: kube_error.to_string(),
        }
    }
}

impl From<InferConfigError> for Error {
    fn from(kube_error: InferConfigError) -> Self {
        Error {
            message: kube_error.to_string(),
        }
    }
}

impl From<KubeconfigError> for Error {
    fn from(kube_error: KubeconfigError) -> Self {
        Error {
            message: kube_error.to_string(),
        }
    }
}
