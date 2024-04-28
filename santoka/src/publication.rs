use crate::TranslatorId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub id: PublicationId,
    pub name: String,
    pub year: Option<u16>,
    pub translator_ids: Vec<TranslatorId>,
    pub description: String,
    pub luca_ranking: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct PublicationId(usize);
