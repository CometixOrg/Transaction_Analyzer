use crate::analytics::transaction_metadata::TransactionMetadata;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TransactionReport {
    pub total_transactions: usize,
    pub total_amount: u64,
    pub average_amount: f64,
    pub transaction_details: HashMap<String, TransactionMetadata>,
}

impl TransactionReport {
    pub fn new() -> Self {
        TransactionReport {
            total_transactions: 0,
            total_amount: 0,
            average_amount: 0.0,
            transaction_details: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, metadata: TransactionMetadata) {
        self.total_transactions += 1;
        self.total_amount += metadata.amount;
        self.transaction_details.insert(metadata.transaction_id.to_string(), metadata);

        self.average_amount = self.total_amount as f64 / self.total_transactions as f64;
    }

    pub fn generate_report(&self) {
        println!("Transaction Report:");
        println!("Total Transactions: {}", self.total_transactions);
        println!("Total Amount: {}", self.total_amount);
        println!("Average Transaction Amount: {}", self.average_amount);
    }
}
