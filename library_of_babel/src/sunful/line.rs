#[derive(Debug, Clone)]
pub struct Line {
    pub absolute_line_number: usize,
    pub text: String,
    pub text_without_comments: String,
}

impl Line {
    pub fn new(absolute_line_number: usize, text: String) -> Self {
        let text_without_comments = text.split("//").next().unwrap().trim().to_string();

        Line {
            absolute_line_number,
            text,
            text_without_comments,
        }
    }

    pub fn is_section_separator(&self) -> bool {
        self.text.trim() == "---"
    }

    pub fn is_comment(&self) -> bool {
        self.text_without_comments.starts_with("//")
    }

    pub fn is_empty(&self) -> bool {
        self.text_without_comments.is_empty()
    }
}