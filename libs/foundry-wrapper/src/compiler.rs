use crate::{types::{WorkspaceConfig, ProjectCompileOutput}, error::Error, utils::install_missing_dependencies};

#[derive(Debug)]
pub struct CompilerInner {
    root_path: String,
    workspace_config: WorkspaceConfig,
}

#[derive(Debug)]
pub struct Compiler {
    inner: CompilerInner,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            inner: CompilerInner {
                root_path: String::new(),
                workspace_config: WorkspaceConfig::new(),
            }
        }
    }

    pub fn load_workspace(&mut self, root_folder: String) -> Result<(), Error> {
        self.inner.workspace_config.load_projects(&root_folder)?;
        self.inner.root_path = root_folder;
        Ok(())
    }

    pub fn reload_project_for_file(&mut self, file_path: &str) -> Result<(), Error> {
        self.inner.workspace_config.reload_project_for_file(file_path)
    }

    pub fn compile(&mut self, file_path: &str) -> Result<(String, ProjectCompileOutput), Error> {
        let (_, config) = self.inner.workspace_config.get_config_for_file(file_path)
            .ok_or_else(|| Error::InvalidFilePath(file_path.to_string()))?;

        if install_missing_dependencies(config, true) {
            let path = self.inner.root_path.clone();
            self.inner.workspace_config.reload_project_for_file(&path)?;
        }
        let (project_path, project) = self.inner.workspace_config.get_project_for_file(file_path)
            .ok_or_else(|| Error::InvalidFilePath(file_path.to_string()))?;

        Ok((project_path, project.compile()?))

        /*
         if install::install_missing_dependencies(&mut config, self.args.silent) &&
            config.auto_detect_remappings
        {
            // need to re-configure here to also catch additional remappings
            config = self.load_config();
            project = config.project()?;
        }

        let filters = self.skip.unwrap_or_default();

        if self.args.silent {
            compile::suppress_compile_with_filter(&project, filters)
        } else {
            let compiler = ProjectCompiler::with_filter(self.names, self.sizes, filters);
            compiler.compile(&project)
        } */
    }
}