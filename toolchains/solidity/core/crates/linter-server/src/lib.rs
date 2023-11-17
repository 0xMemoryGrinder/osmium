use std::cell::RefCell;

use osmium_libs_lsp_handler::{
    lsp_types::{
        Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
        DidOpenTextDocumentParams, InitializeParams, InitializeResult, InitializedParams,
        MessageType, Position, Range, ServerCapabilities, TextDocumentSyncCapability,
        TextDocumentSyncKind, Url, FileChangeType,
    },
    Connection, Handler, Result,
};
use solidhunter_lib::{linter::SolidLinter, types::LintDiag};

struct Backend {
    connection: Connection,
    linter: RefCell<Option<SolidLinter>>,
}

impl Handler for Backend {
    fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        eprintln!("starting example main loop");
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

    fn initialized(&self, _: InitializedParams) {
        eprintln!("Initializing server");
        self.connection
            .log_message(MessageType::INFO, "Server initialized!");

        let linter = SolidLinter::new_fileless();
        self.linter.replace(Some(linter));

        self.connection
            .log_message(MessageType::INFO, "Linter initialized!");
    }

    fn shutdown(&self) -> Result<()> {
        self.connection
            .log_message(MessageType::INFO, "Server shutdown!");
        Ok(())
    }

    fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.connection.log_message(
            MessageType::INFO,
            format!("file opened!: {:}", params.text_document.uri),
        );

        let filepath = filepath_from_uri(&params.text_document.uri);
        let mut linter = self.linter.borrow_mut();
        let linter = match linter.as_mut() {
            Some(l) => l,
            None => {
                eprintln!("Linter cannot be ran due to previous errors");
                return;
            }
        };
        let diags_res = linter.parse_content(&filepath, &params.text_document.text);

        if let Ok(diags) = diags_res {
            let diags = diags
                .diags
                .iter()
                .map(|d| diagnostic_from_lintdiag(d.clone()))
                .collect();
            eprintln!("diags: {:#?}", diags);
            self.connection
                .publish_diagnostics(params.text_document.uri.clone(), diags, None);
        } else if let Err(e) = diags_res {
            self.connection
                .log_message(MessageType::ERROR, e.to_string());
        }
    }

    fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.connection.log_message(
            MessageType::INFO,
            format!("file changed!: {:}", params.text_document.uri),
        );

        let filepath = filepath_from_uri(&params.text_document.uri);
        let mut linter = self.linter.borrow_mut();
        let linter = match linter.as_mut() {
            Some(l) => l,
            None => {
                eprintln!("Linter cannot be ran due to previous errors");
                return;
            }
        };
        let diags_res = linter.parse_content(&filepath, &params.content_changes[0].text);

        if let Ok(diags) = diags_res {
            let diags = diags
                .diags
                .iter()
                .map(|d| diagnostic_from_lintdiag(d.clone()))
                .collect();
            eprintln!("diags: {:#?}", diags);
            self.connection
                .publish_diagnostics(params.text_document.uri.clone(), diags, None);
        } else if let Err(e) = diags_res {
            self.connection
                .log_message(MessageType::ERROR, e.to_string());
        }
    }

    fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        self.connection
            .log_message(MessageType::INFO, "configuration file changed!");

        if params.changes[0].typ == FileChangeType::DELETED {
            return;
        }
        // let mut linter = SolidLinter::new();
        // let res = linter.initialize_rules_content(&params.changes[0].text);
        // if res.is_ok() {
        //     self.linter.replace(Some(linter));
        // }
    }
}

pub fn filepath_from_uri(uri: &Url) -> String {
    let path = uri.path();
    path.to_string()
}

fn diagnostic_from_lintdiag(diag: LintDiag) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position {
                line: diag.range.start.line as u32 - 1,
                character: diag.range.start.character as u32,
            },
            end: Position {
                line: diag.range.end.line as u32 - 1,
                character: diag.range.end.character as u32,
            },
        },
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: Some("osmium-solidity-linter".to_string()),
        message: diag.message,
        related_information: None,
        tags: None,
        data: None,
    }
}

pub fn create_linter(connection: Connection) -> Box<dyn Handler> {
    Box::new(Backend {
        connection,
        linter: RefCell::new(None),
    })
}
