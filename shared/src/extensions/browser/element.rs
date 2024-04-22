use super::*;
use crate::controllers::{controller_name_to_selector, HasProps};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, NodeList};

pub trait ElementExtension {
    fn show(&self);
    fn hide(&self);
    fn find_controllers(&self, controller_name: &str, has_props: HasProps) -> Vec<HtmlElement>;
    fn find_controller(&self, controller_name: &str, has_props: HasProps) -> Option<Element>;
    fn find_controllers_node_list(&self, controller_name: &str, has_props: HasProps) -> NodeList;
    fn parse_props<Props: serde::de::DeserializeOwned>(
        &self,
        controller_name: &str,
    ) -> Option<Props>;
}

impl ElementExtension for Element {
    fn show(&self) {
        self.class_list().remove_1("hidden").unwrap();
    }

    fn hide(&self) {
        self.class_list().add_1("hidden").unwrap();
    }

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
            .expect("element.query_selector_all() failed.")
    }

    fn parse_props<Props: serde::de::DeserializeOwned>(
        &self,
        controller_name: &str,
    ) -> Option<Props> {
        use base64::Engine;
        // controller_class is a sring like:
        // controller:replace:ewogIHJvdXRlOiAiL3BvZW1zLzEyMyIKfQ==
        let class_prefix = crate::controllers::get_class_without_props(controller_name);
        let controller_class = self
            .class_list()
            .into_iter()
            .find(|class| class.as_str().starts_with(&class_prefix))?;
        let separated_by_colons = controller_class.as_str().split(':').collect::<Vec<_>>();
        let base64_props = separated_by_colons.get(2)?;
        let json_bytes = base64::prelude::BASE64_STANDARD
            .decode(base64_props.as_bytes())
            .ok()?;
        let json = String::from_utf8(json_bytes).ok()?;
        serde_json::from_str(&json).ok()
    }
}
