use std::fmt::Display;

pub trait StrExtension {
    fn plus(&self, other: impl Display) -> String;
    fn join_class(&self, other: impl Display) -> String;
}

impl StrExtension for str {
    fn plus(&self, other: impl Display) -> String {
        format!("{}{}", self, other)
    }
    fn join_class(&self, other: impl Display) -> String {
        format!("{} {}", self, other)
    }
}
