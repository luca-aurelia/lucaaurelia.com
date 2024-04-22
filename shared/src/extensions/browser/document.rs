use super::*;
use crate::controllers::{controller_name_to_selector, HasProps};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, NodeList};

pub trait DocumentExtension {
    fn find_controllers(&self, controller_name: &str, has_props: HasProps) -> Vec<HtmlElement>;
    fn find_controller(&self, controller_name: &str, has_props: HasProps) -> Option<Element>;
    fn find_controllers_node_list(&self, controller_name: &str, has_props: HasProps) -> NodeList;
}

impl DocumentExtension for Document {
    fn find_controllers(&self, controller_name: &str, has_props: HasProps) -> Vec<HtmlElement> {
        self.find_controllers_node_list(controller_name, has_props)
            .into_iter()
            .map(|node| node.dyn_into::<HtmlElement>().unwrap())
            .collect()
    }

    fn find_controller(&self, controller_name: &str, has_props: HasProps) -> Option<Element> {
        let selector = controller_name_to_selector(controller_name, has_props);
        self.query_selector(&selector)
            .expect("document.query_selector() failed.")
    }

    fn find_controllers_node_list(&self, controller_name: &str, has_props: HasProps) -> NodeList {
        let selector = controller_name_to_selector(controller_name, has_props);
        self.query_selector_all(&selector)
            .expect("document.query_selector_all() failed.")
    }
}
