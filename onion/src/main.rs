mod account_name;
mod crate_wrappers;
mod institution;
mod personal_or_business;
mod predictability;
mod standard_transaction;
mod tiller_transaction;

use camino::Utf8PathBuf;
use crate_wrappers::jiff::DateExtension;
use standard_transaction::*;
use tiller_transaction::TillerTransaction;

fn main() {
    println!();

    let input_csv = include_str!("/Users/photon-garden/Downloads/Tiller - Transactions.csv");
    let transactions = get_last_weeks_transactions(input_csv);
    save_transactions_to_obsidian(input_csv, transactions);
}

fn get_last_weeks_transactions(csv: &str) -> Vec<StandardTransaction> {
    println!("Getting transactions from CSV");

    let transactions: Vec<_> = TillerTransaction::transactions_from_csv(csv)
        .into_iter()
        .filter(|transaction| transaction.date.happened_last_week())
        .map(StandardTransaction::from_tiller_transaction)
        .collect();

    println!("Got {} transactions from CSV", transactions.len());

    transactions
}

fn save_transactions_to_obsidian(
    tiller_csv: &str,
    standard_transactions: Vec<StandardTransaction>,
) {
    let monday_of_last_week = jiff::civil::Date::monday_of_last_week();
    let transactions_dir_path =
        paths::path_to_obsidian_transactions_folder(monday_of_last_week.year());

    save_tiller_csv_to_obsidian(tiller_csv, &transactions_dir_path);
    save_standard_transactions_to_obsidian(
        standard_transactions,
        &transactions_dir_path,
        &monday_of_last_week,
    );
}

fn save_tiller_csv_to_obsidian(tiller_csv: &str, transactions_dir_path: &Utf8PathBuf) {
    let tiller_csv_path = transactions_dir_path.join("Latest Tiller transactions.csv");
    println!("Saving Tiller transactions to {}", tiller_csv_path);
    std::fs::write(tiller_csv_path, tiller_csv).expect("Unable to write Tiller CSV file");
}

fn save_standard_transactions_to_obsidian(
    standard_transactions: Vec<StandardTransaction>,
    transactions_dir_path: &Utf8PathBuf,
    monday_of_last_week: &jiff::civil::Date,
) {
    let transactions_file_name = format!("Transactions, week of {}.csv", monday_of_last_week);
    let last_weeks_transactions_csv_path = transactions_dir_path.join(transactions_file_name);

    println!(
        "Saving transactions to {}",
        last_weeks_transactions_csv_path
    );

    let file = std::fs::File::create(last_weeks_transactions_csv_path)
        .expect("Unable to create transactions CSV file");

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    for transaction in standard_transactions {
        writer
            .serialize(transaction)
            .expect("Unable to write record to CSV file");
    }

    writer.flush().expect("Unable to flush CSV writer");

    println!("Transactions saved!");
}
