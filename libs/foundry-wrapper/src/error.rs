use foundry_compilers::error::SolcError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config loading error: {0}")]
    ConfigLoading(SolcError),
    #[error("Project loading error: {0}")]
    ProjectLoading(SolcError),
    #[error("Workspace loading error: {0}")]
    InvalidRootPath(#[from] glob::PatternError),
    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),
    #[error("Compilation error: {0}")]
    CompilationError(SolcError),
    #[error("Unkown error: {0}")]
    UnkownError(#[from] SolcError),
}