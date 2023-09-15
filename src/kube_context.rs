use std::fmt::Display;

use iced::{
    widget::{horizontal_space, row, text},
    Element,
};
use kube::Config;

use crate::{colours, messages::Message, sizes};

#[derive(Debug, Clone)]
pub struct KubeContext {
    config: Config,
    namespace: String,
}

impl KubeContext {
    pub fn new(config: Config, namespace: String) -> KubeContext {
        KubeContext { config, namespace }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            text("Cluster:").style(colours::get_grey()),
            text(self.config.cluster_url.to_string()).style(colours::get_white()),
            horizontal_space(sizes::SEP),
            text("Namespace:").style(colours::get_grey()),
            text(self.namespace.clone()).style(colours::get_white()),
        ]
        .spacing(sizes::SEP)
        .into()
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_namespace(&self) -> String {
        self.namespace.clone()
    }
}

impl Display for KubeContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.config.auth_info.username.as_ref() {
            Some(username) => {
                write!(
                    f,
                    "{}/{}/{}",
                    self.config.cluster_url, self.config.default_namespace, username
                )
            }
            None => {
                write!(
                    f,
                    "{}/{}",
                    self.config.cluster_url, self.config.default_namespace
                )
            }
        }
    }
}
