use crate::account_name::AccountName;

#[derive(Debug, serde::Serialize)]
pub enum Predictability {
    Predictable,
    NonPredictable,
    Unknown,
}

impl Predictability {
    pub fn from_account_name(account_name: &AccountName) -> Self {
        match account_name {
            AccountName::PhotonGardenPredictableExpenses => Self::Predictable,
            AccountName::AllyNonPredictableExpenses
            | AccountName::AllyPredictableExpenses
            | AccountName::PhotonGardenNonPredictableExpenses
            | AccountName::CapitalOneQuicksilver
            | AccountName::Discover
            | AccountName::Cash => Self::NonPredictable,
            AccountName::CapitalOneVenture => Self::Unknown,
        }
    }
}
