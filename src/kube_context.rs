use iced::{Element, widget::{row, text, horizontal_space}};
use k8s_openapi::http::Uri;
use kube::Config;

use crate::{messages::Message, colours, sizes};

#[derive(Debug, Clone)]
pub struct KubeContext {
    config: Config,
}

impl KubeContext {
    pub fn new(config: Config) -> KubeContext {
        KubeContext {
            config
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            text("Cluster:").style(colours::get_grey()),
            text(self.config.cluster_url.to_string()),
            horizontal_space(sizes::SEP),
            text("Namespace:").style(colours::get_grey()),
            text(self.config.default_namespace.clone()),
        ].into()
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}
