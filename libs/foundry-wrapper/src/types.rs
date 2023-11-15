pub(crate) mod project_compile_output;
pub use project_compile_output::{ProjectCompileOutput, CompilationError, Position, Range, Severity};

pub(crate) mod project_config;
pub use project_config::ProjectConfig;

pub(crate) mod project;
pub use project::Project;

pub(crate) mod workspace_config;
pub use workspace_config::WorkspaceConfig;