use crate::account_name::AccountName;

#[derive(Debug, serde::Serialize)]
pub enum PersonalOrBusiness {
    Personal,
    Business,
}

impl PersonalOrBusiness {
    pub fn from_account_name(account_name: &AccountName) -> Self {
        match account_name {
            AccountName::PhotonGardenPredictableExpenses
            | AccountName::PhotonGardenNonPredictableExpenses => Self::Business,

            AccountName::AllyPredictableExpenses
            | AccountName::AllyNonPredictableExpenses
            | AccountName::CapitalOneQuicksilver
            | AccountName::CapitalOneVenture
            | AccountName::Discover
            | AccountName::Cash => Self::Personal,
        }
    }
}
