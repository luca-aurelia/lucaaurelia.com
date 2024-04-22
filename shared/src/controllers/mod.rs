// pub mod replace;
pub mod parallax;
pub mod replace;
pub mod show_hide;
pub mod show_if_scrolled;

static CONTROLLER_PREFIX: &str = "controller";

pub fn get_class_without_props(controller_name: &str) -> String {
    format!(
        "{prefix}:{name}",
        prefix = CONTROLLER_PREFIX,
        name = controller_name,
    )
}

pub fn get_class_with_props<Props: serde::Serialize>(
    controller_name: &str,
    props: Props,
) -> String {
    use base64::Engine;

    let json_props = serde_json::to_string(&props).expect("Failed to serialize props.");
    let base64_props = base64::prelude::BASE64_STANDARD.encode(json_props.as_bytes());

    format!(
        "{prefix}:{name}:{props}",
        prefix = CONTROLLER_PREFIX,
        name = controller_name,
        props = base64_props
    )
}

/// This function's logic will be incorrect if controller_name is a target like `controller:show-hide:toggle`.
pub fn controller_name_to_selector(controller_name: &str, has_props: HasProps) -> String {
    // format!(".{}", component_name).replace(':', "\\:")

    let result = match has_props {
        HasProps::Yes => format!(
            // This matches any element who has a class starting
            // with `component_name`. This is important for controllers
            // with props, which are marked by a class like this:
            //
            // controller:replace:ewogIHJvdXRlOiAiL3BvZW1zLzEyMyIKfQ==
            //
            // Where the random-looking string at the end is the base64
            // encoding of the props.
            //
            // In contrast, controllers without props are marked by a class
            // like this:
            //
            // controller:show-hide
            "[class*='controller:{name}:']",
            name = controller_name,
        ),
        HasProps::No => format!(
            "[class^='controller:{name} '], [class*=' controller:{name} '], [class$=' controller:{name}'], [class='controller:{name}']",
            name = controller_name,
        ),
    };
    println!("{}", &result);
    result
}

pub enum HasProps {
    Yes,
    No,
}
