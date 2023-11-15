use foundry_config::Config;
use crate::error::Error;

use super::Project;

#[derive(Clone, Debug)]
pub(crate) struct ProjectConfigInner(pub Config);

impl ProjectConfigInner {
    pub(crate) fn load_with_root(root: &str) -> Self {
        Self(Config::load_with_root(root))
    }

    pub(crate) fn get_mut(&mut self) -> &mut Config {
        &mut self.0
    }
}

#[derive(Clone, Debug)]
pub struct ProjectConfig {
    pub(crate) inner: ProjectConfigInner,
}

impl ProjectConfig {
    pub fn load_with_root(root: &str) -> Self {
        Self {
            inner: ProjectConfigInner::load_with_root(root),
        }
    }

    pub fn project(&self) -> Result<Project, Error> {
        Ok(Project::new(self)?)
    }
}