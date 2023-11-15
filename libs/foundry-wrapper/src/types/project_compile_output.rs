#[derive(Clone, Debug)]
pub struct ProjectCompileOutput(foundry_compilers::ProjectCompileOutput);

impl From<foundry_compilers::ProjectCompileOutput> for ProjectCompileOutput {
    fn from(output: foundry_compilers::ProjectCompileOutput) -> Self {
        Self(output)
    }
}

impl ProjectCompileOutput {
    pub fn get_errors(&self) -> Vec<CompilationError> {
        self.0.clone().output().errors.into_iter().map(|e| CompilationError::from(e)).collect()
    }
}

#[derive(Clone, Debug)]
pub struct CompilationError(foundry_compilers::artifacts::Error);

impl From<foundry_compilers::artifacts::Error> for CompilationError {
    fn from(error: foundry_compilers::artifacts::Error) -> Self {
        Self(error)
    }
}

impl CompilationError {
    pub fn get_message(&self) -> String {
        self.0.message.clone()
    }

    pub fn get_file_path(&self) -> Option<String> {
        Some(self.0.source_location.clone()?.file.clone())
    }

    pub fn get_start_idx(&self) -> Option<i32> {
        Some(self.0.source_location.clone()?.start)
    }

    pub fn get_end_idx(&self) -> Option<i32> {
        Some(self.0.source_location.clone()?.end)
    }

    pub fn get_start_position(&self, source_content: &str) -> Option<Position> {
        let idx = self.get_start_idx()?;
        Position::from_index(idx, source_content)
    }

    pub fn get_end_position(&self, source_content: &str) -> Option<Position> {
        let idx = self.get_end_idx()?;
        Position::from_index(idx, source_content)
    }

    pub fn get_range(&self, source_content: &str) -> Option<Range> {
        Some(Range {
            start: self.get_start_position(source_content)?,
            end: self.get_end_position(source_content)?
        })
    }

    pub fn get_severity(&self) -> Severity {
        self.0.severity.into()
    }
}

/**
 * Position of error, 0 based indexes
 */
#[derive(Clone, Debug)]
pub struct Position {
    pub line: u32,
    pub column: u32
}

impl Position {
    pub fn from_index(idx: i32, source: &str) -> Option<Self> {
        let mut idx: usize = idx as usize;
        for (i, l) in source.lines().enumerate() {
            if idx < l.len() {
                return Some(Self {
                    line: i as u32,
                    column: idx as u32
                })
            }
            idx -= l.len() + 1;
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position
}

#[derive(Clone, Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl From<foundry_compilers::artifacts::Severity> for Severity {
    fn from(severity: foundry_compilers::artifacts::Severity) -> Self {
        match severity {
            foundry_compilers::artifacts::Severity::Error => Self::Error,
            foundry_compilers::artifacts::Severity::Warning => Self::Warning,
            foundry_compilers::artifacts::Severity::Info => Self::Info,
        }
    }
}