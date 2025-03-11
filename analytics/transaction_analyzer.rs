use crate::analytics::transaction_metadata::TransactionMetadata;
use crate::analytics::transaction_report::TransactionReport;
use crate::analytics::transaction_history::TransactionHistory;
use crate::analytics::transaction_metrics::TransactionMetrics;

pub struct TransactionAnalyzer {
    report: TransactionReport,
    history: TransactionHistory,
    metrics: TransactionMetrics,
}

impl TransactionAnalyzer {
    pub fn new(history_size: usize) -> Self {
        TransactionAnalyzer {
            report: TransactionReport::new(),
            history: TransactionHistory::new(history_size),
            metrics: TransactionMetrics::new(),
        }
    }

    pub fn analyze_transaction(&mut self, metadata: TransactionMetadata) {
        self.report.add_transaction(metadata.clone());
        self.history.add_transaction(metadata.clone());
        self.metrics.add_transaction(metadata);
        self.report.generate_report();
        self.history.print_history();
        self.metrics.print_metrics();
    }
}
