use crate::error::Error;

use super::{ProjectConfig, ProjectCompileOutput};

#[derive(Debug)]
pub(crate) struct ProjectInner(foundry_compilers::Project);

#[derive(Debug)]
pub struct Project {
    inner: ProjectInner,
}

impl Project {
    pub(super) fn new(config: &ProjectConfig) -> Result<Self, Error> {
        Ok(Self {
            inner: ProjectInner(
                config.inner.0.project().map_err(|e| Error::ProjectLoading(e))?
            ),
        })
    }

    pub(crate) fn compile(&self) -> Result<ProjectCompileOutput, Error> {
        Ok(self.inner.0.compile().map_err(|e| Error::UnkownError(e))?.into())
    }
}