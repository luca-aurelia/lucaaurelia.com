use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Translator {
    pub id: TranslatorId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct TranslatorId(usize);
