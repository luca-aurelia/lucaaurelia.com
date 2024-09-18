#[derive(Debug, serde::Serialize)]
pub enum Institution {
    Ally,
    CapitalOne,
    Discover,
    Mercury,
    Cash,
}

impl Institution {
    pub fn from_str(name: &str) -> Self {
        match name {
            "Ally Bank" => Self::Ally,
            "Capital One" => Self::CapitalOne,
            "Discover" => Self::Discover,
            "Mercury Banking" => Self::Mercury,
            "Cash" => Self::Cash,
            _ => panic!("Unknown institution: {}", name),
        }
    }
}
