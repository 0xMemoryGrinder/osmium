use std::collections::HashMap;
use super::{ProjectConfig, Project};
use crate::{utils::find_projects_paths, error::Error};


#[derive(Debug)]
pub(crate) struct WorkspaceConfigInner {
    paths_to_config_and_project: HashMap<String, (ProjectConfig, Project)>,
}

#[derive(Debug)]
pub struct WorkspaceConfig {
    inner: WorkspaceConfigInner,
}

impl WorkspaceConfig {
    pub fn new() -> Self {
        Self {
            inner: WorkspaceConfigInner {
                paths_to_config_and_project: HashMap::new(),
            }
        }
    }

    pub fn load_projects(&mut self, root_folder: &str) -> Result<(), Error> {
        let paths = find_projects_paths(&root_folder)?;
        for path in paths {
            if let Some(path) = path.to_str() {
                self.add_project(path)?;
            }
        }
        Ok(())
    }

    pub fn reload_project_for_file(&mut self, file_path: &str) -> Result<(), Error> {
        if let Some(project_path) = self.project_path_for_file(file_path) {
            self.add_project(&project_path)?;
        }
        Ok(())
    }

    fn add_project(&mut self, path: &str) -> Result<(), Error> {
        let config = ProjectConfig::load_with_root(path);
        let project = config.project()?;
        self.inner.paths_to_config_and_project.insert(path.to_string(), (config, project));
        Ok(())
    }

    pub fn project_path_for_file(&self, file_path: &str) -> Option<String> {
        let path = self.inner.paths_to_config_and_project
            .keys()
            .filter(|path| file_path.starts_with(path.as_str()))
            .max_by_key(|path| path.len())?;
        Some(path.to_string())
    }

    pub fn get_project(&mut self, project_path: &str) -> Option<&mut Project> {
        self.inner.paths_to_config_and_project.get_mut(project_path).map(|(_, project)| project)
    }

    pub fn get_project_for_file(&mut self, file_path: &str) -> Option<(String, &mut Project)> {
        self.project_path_for_file(file_path).and_then(|path| Some((path.clone(), self.get_project(&path)?)))
    }

    pub fn get_config(&mut self, project_path: &str) -> Option<&mut ProjectConfig> {
        self.inner.paths_to_config_and_project.get_mut(project_path).map(|(config, _)| config)
    }

    pub fn get_config_for_file(&mut self, file_path: &str) -> Option<(String, &mut ProjectConfig)> {
        self.project_path_for_file(file_path).and_then(|path| Some((path.clone(), self.get_config(&path)?)))
    }

    pub fn get(&mut self, project_path: &str) -> Option<&mut (ProjectConfig, Project)> {
        self.inner.paths_to_config_and_project.get_mut(project_path)
    }

    pub fn get_for_file(&mut self, file_path: &str) -> Option<(String, &mut (ProjectConfig, Project))> {
        self.project_path_for_file(file_path)
            .and_then(|path| Some((path.clone(), self.inner.paths_to_config_and_project.get_mut(&path)?)))
    }
}