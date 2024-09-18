#[derive(Debug, serde::Deserialize)]
pub struct TillerTransaction {
    #[serde(rename = "Date")]
    pub date: jiff::civil::Date,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Account")]
    pub account_name: String,
    #[serde(rename = "Institution")]
    pub institution: String,
    #[serde(rename = "Transaction ID")]
    pub transaction_id: String,
}

impl TillerTransaction {
    pub fn transactions_from_csv(csv: &str) -> Vec<Self> {
        let cursor = std::io::Cursor::new(csv);
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(cursor);

        reader
            .deserialize()
            .map(|result| result.expect("Error deserializing record"))
            .collect()
    }
}
