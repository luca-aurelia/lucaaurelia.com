use super::line::Line;
use super::normalized_string::NormalizedString;
use super::parse_error::ParseError;
use super::raw_section::RawSection;

#[derive(Debug)]
pub struct Schema {
    pub expected_fields: Vec<FieldDefinition>,
}

const SCHEMA_LANGUAGE: &str = "sunful-schema";

impl Schema {
    pub fn contains_schema_definition(raw_section: &RawSection) -> bool {
        let code_block_opening = format!("```{}", SCHEMA_LANGUAGE);
        raw_section
            .text
            .to_lowercase()
            .contains(&code_block_opening)
    }

    pub fn from_raw_section(raw_section: &RawSection) -> Result<Self, ParseError> {
        if !Self::contains_schema_definition(raw_section) {
            return Err(ParseError::NoSchemaDefinition);
        }

        let schema_lines = raw_section
            .find_code_block(SCHEMA_LANGUAGE)
            .ok_or(ParseError::NoSchemaDefinition)?;

        type FieldDefinitions = Vec<FieldDefinition>;

        let expected_fields = schema_lines
            .into_iter()
            .filter(|line| !line.is_comment())
            .map(FieldDefinition::from_line)
            .collect::<Result<FieldDefinitions, ParseError>>()?;

        Ok(Schema { expected_fields })
    }

    pub fn required_fields(&self) -> impl Iterator<Item = &FieldDefinition> {
        self.expected_fields
            .iter()
            .filter(|field| field.required == Required::Yes)
    }

    pub fn field_definition(&self, name: &str) -> Option<&FieldDefinition> {
        let normalized_name = NormalizedString::new(name);
        self.expected_fields
            .iter()
            .find(|field| field.name == normalized_name)
    }
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: NormalizedString,
    pub required: Required,
    pub expected_type: ExpectedType,
}

impl FieldDefinition {
    fn from_line(line: &Line) -> Result<Self, ParseError> {
        let mut parts = line.text_without_comments.split(':');

        let name = parts
            .next()
            .ok_or_else(|| ParseError::field_definition_missing_name(line))?;

        let type_text = parts
            .next()
            .ok_or_else(|| ParseError::field_definition_missing_type(line))?
            .trim();

        let (expected_type, required) = parse_type_that_might_be_optional(line, type_text)?;

        Ok(FieldDefinition {
            name: NormalizedString::new(name),
            required,
            expected_type,
        })
    }
}

#[derive(Debug, Clone, enum_iterator::Sequence)]
pub enum ExpectedType {
    String,
    U64,
    Link,
}

impl ExpectedType {
    fn as_str(&self) -> &'static str {
        match self {
            ExpectedType::String => "string",
            ExpectedType::U64 => "u64",
            ExpectedType::Link => "link",
        }
    }
}

fn parse_type_that_might_be_optional(
    line: &Line,
    type_text_maybe_with_prefix: &str,
) -> Result<(ExpectedType, Required), ParseError> {
    let optional = "optional ";

    let (type_text, required) = match type_text_maybe_with_prefix.strip_prefix(optional) {
        Some(without_prefix) => (without_prefix, Required::No),
        None => (type_text_maybe_with_prefix, Required::Yes),
    };

    let expected_type = parse_type(line, type_text)?;
    Ok((expected_type, required))
}

fn parse_type(line: &Line, type_text: &str) -> Result<ExpectedType, ParseError> {
    let all_expected_types = enum_iterator::all::<ExpectedType>();
    for expected_type in all_expected_types {
        if expected_type.as_str() == type_text {
            return Ok(expected_type);
        }
    }

    Err(ParseError::unexpected_field_type(line, type_text))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Required {
    Yes,
    No,
}
