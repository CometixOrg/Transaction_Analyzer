pub mod core {
    pub mod transaction_exec;
    pub mod flow_controller;
    pub mod nested_resolver;
}

pub mod runtime {
    pub mod analyzer_runtime;
    pub mod state_models;
}

pub mod ops {
    pub mod context_spawner;
    pub mod allocator;
}

pub mod proto {
    pub mod instruction;
    pub mod error;
}

pub use core::transaction_exec::execute_instruction;
