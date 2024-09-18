use crate::account_name::AccountName;
use crate::institution::Institution;
use crate::personal_or_business::PersonalOrBusiness;
use crate::predictability::Predictability;
use crate::tiller_transaction::TillerTransaction;

#[derive(Debug, serde::Serialize)]
pub struct StandardTransaction {
    /// Manually added by me.
    pub description: String,
    pub date: jiff::civil::Date,
    /// Maps to the Description provided by Ally Bank.
    pub original_description: String,
    /// The amount of the transaction.
    pub amount: f64,
    /// The amount I paid, after any splitting with others.
    pub my_amount: f64,
    pub predictability: Predictability,
    pub account_name: AccountName,
    pub personal_or_business: PersonalOrBusiness,
    pub transaction_id: String,
}

impl StandardTransaction {
    pub fn from_tiller_transaction(tiller_transaction: TillerTransaction) -> Self {
        let institution = Institution::from_str(&tiller_transaction.institution);
        let account_name = AccountName::from_insitution_and_account_name(
            institution,
            &tiller_transaction.account_name,
        );
        let predictability = Predictability::from_account_name(&account_name);
        let personal_or_business = PersonalOrBusiness::from_account_name(&account_name);

        Self {
            description: "".to_string(),
            date: tiller_transaction.date,
            original_description: tiller_transaction.description,
            amount: tiller_transaction.amount,
            my_amount: tiller_transaction.amount,
            predictability,
            account_name,
            personal_or_business,
            transaction_id: tiller_transaction.transaction_id,
        }
    }
}
