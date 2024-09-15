use super::line::Line;

#[derive(Debug)]
pub struct RawSection {
    pub starting_line_number: usize,
    pub lines: Vec<Line>,
    pub text: String,
}

impl RawSection {
    pub fn new(lines: Vec<Line>) -> Self {
        let starting_line_number = lines.first().unwrap().absolute_line_number;
        let text = lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<String>();
        RawSection {
            starting_line_number,
            lines,
            text,
        }
    }

    /// This returns the lines of the first code block with the given language.
    /// It excludes the starting (```lang) and ending (```) lines.
    pub fn find_code_block(&self, language: &str) -> Option<&[Line]> {
        let code_block_start = format!("```{}", language);
        let code_block_end = "```";

        let code_block_start_index = self
            .lines
            .iter()
            .position(|line| line.text.trim() == code_block_start)?;
        dbg!(code_block_start_index);

        let code_block_end_index = self.lines[code_block_start_index..]
            .iter()
            .position(|line| line.text.trim() == code_block_end)
            .map(|index| code_block_start_index + index)?;
        dbg!(code_block_end_index);

        // We start the slice at code_block_start_index + 1 to exclude the starting line (```lang).
        let code_block = &self.lines[(code_block_start_index + 1)..code_block_end_index];
        Some(code_block)
    }
}
