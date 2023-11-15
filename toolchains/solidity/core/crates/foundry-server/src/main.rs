use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use osmium_libs_foundry_wrapper::{Compiler, ProjectCompileOutput, CompilationError};
mod utils;
use utils::{get_root_path, convert_severity};
mod affected_fiiles_store;
use affected_fiiles_store::AffectedFilesStore;

#[derive(Debug)]
struct State {
    compiler: Compiler,
    initialized: bool,
    affected_files: AffectedFilesStore,
}

#[derive(Debug)]
struct Backend {
    client: Client,
    state: Mutex<State>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.client
            .log_message(MessageType::INFO, "Foundry server initializing!")
            .await;
        let opt_path = get_root_path(params.clone());
        if let Some(path) = opt_path {
            self.client
                .log_message(MessageType::INFO, &format!("Foundry server initializing with workspace path: {:?}", path))
                .await;
            self.load_workspace(path).await;
        } else {
            self.client
                .log_message(MessageType::INFO, "Foundry server not initialized : no workspace path!")
                .await;
        }
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "foundryserver initialized!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("file opened!: {:}", params.text_document.uri))
            .await;
        self.compile(params.text_document.uri.path().to_string()).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("file changed!: {:}", params.text_document.uri))
            .await;
        self.compile(params.text_document.uri.path().to_string()).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    pub async fn load_workspace(&self, path: String) {
        let mut state = self.state.lock().await;
        if let Err(err) = state.compiler.load_workspace(path) {
            self.client
                .log_message(MessageType::ERROR, &format!("Foundry server failed to initialize: {:?}", err))
                .await;
        } else {
            state.initialized = true;
            self.client
                .log_message(MessageType::INFO, "Foundry server initialized!")
                .await;
        }
        drop(state);
    }

    pub async fn compile(&self, filepath: String) {
        let mut state = self.state.lock().await;
        if !state.initialized {
            // unlock the mutex before calling load_workspace
            drop(state);
            self.client
                .log_message(MessageType::INFO, "Foundry server initializing!")
                .await;
            let folder_path = Path::new(&filepath).parent().unwrap().to_str().unwrap().to_string();
            self.load_workspace(folder_path).await;
            state = self.state.lock().await;
        }
        self.client
            .log_message(MessageType::INFO, "Foundry server compiling!")
            .await;
        let output = state.compiler.compile(&filepath);
        match output {
            Ok((project_path, output)) => {
                self.client
                    .log_message(MessageType::INFO, format!("Compile errors: {:?}", output.get_errors()))
                    .await;
                drop(state);
                self.publish_errors_diagnostics(project_path, filepath, output).await;
            }
            Err(err) => {
                self.client
                    .log_message(MessageType::ERROR, format!("error: {:?}", err))
                    .await;
            }
        }
        
    }

    pub async fn publish_errors_diagnostics(&self, project_path: String, filepath: String, output: ProjectCompileOutput) {
        let mut diagnostics = HashMap::<Url, Vec<Diagnostic>>::new();
        for error in output.get_errors() {
            let (source_content_filepath, range) = match self.extract_diagnostic_range(&project_path, &error).await {
                Some((source_content_filepath, range)) => (source_content_filepath, range),
                None => continue,
            };
            let diagnostic = Diagnostic {
                range: Range {
                    start: Position {
                        line: range.start.line,
                        character: range.start.column,
                    },
                    end: Position {
                        line: range.end.line,
                        character: range.end.column,
                    },
                },
                severity: Some(convert_severity(error.get_severity())),
                code: None,
                code_description: None,
                source: Some("foundry".to_string()),
                message: error.get_message(),
                related_information: None,
                tags: None,
                data: None,
            };
            let url = Url::parse(&format!("file://{}", source_content_filepath.to_str().unwrap())).unwrap();
            if !diagnostics.contains_key(&url) {
                diagnostics.insert(url.clone(), vec![diagnostic]);
            } else {
                diagnostics.get_mut(&url).unwrap().push(diagnostic);
            }
        }

        self.add_not_affected_files(project_path, filepath, &mut diagnostics).await;
        for (uri, diags) in diagnostics.iter() {
            self.client
                .publish_diagnostics(uri.clone(), diags.clone(), None)
                .await;
        }
    
    }

    async fn extract_diagnostic_range(&self, project_path: &str, error: &CompilationError) -> Option<(PathBuf, osmium_libs_foundry_wrapper::Range)> {
        let source_content_filepath = match error.get_file_path() {
            Some(source_path) => {
                let mut complete_path = Path::new(project_path).to_path_buf();
                complete_path.push(source_path);
                complete_path
            }
            None =>  {
                self.client
                    .log_message(MessageType::ERROR, format!("error, cannot get filepath: {:?}", error))
                    .await;
                return None;
            }
        };
        let source_content = match std::fs::read_to_string(&source_content_filepath) {
            Ok(content) => content,
            Err(err) => {
                self.client
                    .log_message(MessageType::ERROR, format!("error, cannot read file: {:?}, error: {:?}", &source_content_filepath, err))
                    .await;
                return None;
            }
        };
        let range = match error.get_range(&source_content) {
            Some(range) => range,
            None => {
                self.client
                    .log_message(MessageType::ERROR, format!("error, cannot get range: {:?}", error))
                    .await;
                return None;
            }
        };
        Some((source_content_filepath, range))
    }

    async fn add_not_affected_files(&self, project_path: String, filepath: String, files: &mut HashMap<Url, Vec<Diagnostic>>) {
        let mut state = self.state.lock().await;

        state.affected_files.add_project_file(project_path.clone(), filepath.clone());

        let affected_files = state.affected_files.get_affected_files(&project_path);
        drop(state);
        for file in affected_files {
            let url = Url::parse(&format!("file://{}", file)).unwrap();
            if !files.contains_key(&url) {
                self.client
                    .log_message(MessageType::INFO, format!("file without diagnostic: {:?}", file))
                    .await;
                files.insert(url, vec![]);
            }
        }
    }


}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| 
        Backend { 
            client, 
            state: Mutex::new(State {
                compiler: Compiler::new(), 
                initialized: false,
                affected_files: AffectedFilesStore::new(),
            })
        }
    );
    Server::new(stdin, stdout, socket).serve(service).await;
}

