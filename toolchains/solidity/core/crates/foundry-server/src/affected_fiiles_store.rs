use std::collections::HashMap;

#[derive(Debug)]
pub struct AffectedFilesStore {
    projects_files: HashMap<String, Vec<String>>,
}

impl AffectedFilesStore {
    pub fn new() -> Self {
        Self {
            projects_files: HashMap::new(),
        }
    }

    pub fn add_project_file(&mut self, project_path: String, file: String) {
        if !self.projects_files.contains_key(&project_path) {
            self.projects_files.insert(project_path.clone(), vec![]);
        } else {
            let files = self.projects_files.get_mut(&project_path).unwrap();
            if !files.contains(&file) {
                files.push(file);
            }
        }
    }

    pub fn get_affected_files(&self, project_path: &str) -> Vec<String> {
        self.projects_files.get(project_path).unwrap().clone()
    }
}