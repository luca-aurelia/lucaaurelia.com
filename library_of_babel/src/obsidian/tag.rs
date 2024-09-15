use lazy_static::lazy_static;
use regex::Regex;

pub struct Tag {}

impl Tag {
    pub fn parse_tags(string: &str) -> Vec<String> {
        lazy_static! {
            static ref MATCH_TAGS: Regex = Regex::new(r"#(\S+)").expect("Error compiling regex.");
        }

        MATCH_TAGS
            .captures_iter(string)
            // capture[0] gives us the entire match, but we want the first
            // capture group, which is why we do capture[1]
            .map(|capture| capture[1].to_string())
            .collect()
    }
}
