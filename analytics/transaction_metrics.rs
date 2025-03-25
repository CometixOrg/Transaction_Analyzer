use crate::analytics::transaction_metadata::TransactionMetadata;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TransactionMetrics {
    pub transaction_count: HashMap<String, usize>,
    pub total_amount: HashMap<String, u64>,
}

impl TransactionMetrics {
    pub fn new() -> Self {
        TransactionMetrics {
            transaction_count: HashMap::new(),
            total_amount: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, metadata: TransactionMetadata) {
        let source = metadata.source.to_string();
        let destination = metadata.destination.to_string();

        *self.transaction_count.entry(source.clone()).or_insert(0) += 1;
        *self.total_amount.entry(source.clone()).or_insert(0) += metadata.amount;

        *self.transaction_count.entry(destination.clone()).or_insert(0) += 1;
        *self.total_amount.entry(destination.clone()).or_insert(0) += metadata.amount;
    }

    pub fn print_metrics(&self) {
        println!("Transaction Metrics:");
        for (address, count) in self.transaction_count.iter() {
            println!("Address: {}, Transactions: {}, Total Amount: {}", address, count, self.total_amount.get(*address).unwrap_or(&0));
        }
    }
}
