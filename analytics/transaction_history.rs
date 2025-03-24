use crate::analytics::transaction_metadata::TransactionMetadata;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TransactionHistory {
    history: VecDeque<TransactionMetadata>,
    pub max_history_size: usize,
}

impl TransactionHistory {
    pub fn new(max_history_size: usize) -> Self {
        TransactionHistory {
            history: VecDeque::with_capacity(max_history_size),
            max_history_size,
        }
    }

    pub fn add_transaction(&mut self, metadata: TransactionMetadata) {
        if self.history.len() == self.max_history_size {
            self.history.pop_front();
        }
        self.history.push_back(metadata);
    }

    pub fn print_history(&self) {
        println!("Transaction History:");
        for metadata in self.history.iter() {
            println!("{:?}", metadata);
        }
    }
}
