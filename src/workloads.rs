use crate::cluster_object::ClusterObject;
use crate::colours;
use crate::sizes;
use crate::Message;
use iced::widget::{button, column, container, horizontal_space, row, text, vertical_rule};
use iced::{Alignment, Element, Length, Padding};

#[derive(Debug, Clone, Default)]
pub struct Workloads {
    cluster_objects: Vec<ClusterObject>,
}

impl Workloads {
    pub fn new(
        cluster_objects: Vec<ClusterObject>,
    ) -> Workloads {
        Workloads { cluster_objects }
    }

    pub fn view(&self) -> Element<Message> {
        let workload_elts: Vec<Element<Message>> = self
            .cluster_objects
            .iter()
            .map(|cluster_object| cluster_object.view())
            .collect();

        column(workload_elts).spacing(sizes::SEP).into()
    }
}

// impl Default for Workloads {

//     fn default() -> Self {
//         Workloads { cluster_objects: vec![] }
//     }

// }
