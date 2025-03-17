use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum AnalyzerPipelineError {
    #[error("The owner of the record does not match the expected subject.")]
    MismatchedOwner,
}

impl From<AnalyzerPipelineError> for ProgramError {
    fn from(e: AnalyzerPipelineError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
