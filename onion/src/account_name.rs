use crate::institution::Institution;

#[derive(Debug, serde::Serialize)]
pub enum AccountName {
    AllyPredictableExpenses,
    AllyNonPredictableExpenses,
    CapitalOneVenture,
    CapitalOneQuicksilver,
    Discover,
    PhotonGardenPredictableExpenses,
    PhotonGardenNonPredictableExpenses,
    Cash,
}

impl AccountName {
    pub fn from_insitution_and_account_name(institution: Institution, account_name: &str) -> Self {
        use crate::institution::Institution;

        match institution {
            Institution::Ally => match account_name {
                "Predictable Expenses" => Self::AllyPredictableExpenses,
                "Non-Predictable Expenses" => Self::AllyNonPredictableExpenses,
                _ => panic!("Unknown account name for Ally Bank: {}", account_name),
            },
            Institution::CapitalOne => match account_name {
                "Venture" => Self::CapitalOneVenture,
                "Quicksilver" => Self::CapitalOneQuicksilver,
                _ => panic!("Unknown account name for Capital One: {}", account_name),
            },
            Institution::Discover => Self::Discover,
            Institution::Mercury => match account_name {
                "Luca: Predictable Expenses" => Self::PhotonGardenPredictableExpenses,
                "Luca: Non-Predictable Expenses" => Self::PhotonGardenNonPredictableExpenses,
                _ => panic!("Unknown account name for Mercury: {}", account_name),
            },
            Institution::Cash => Self::Cash,
        }
    }
}
