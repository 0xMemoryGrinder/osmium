use crate::connection::Connection;
use crate::handler::Handler;
use crate::helpers::log;
use crate::jsonrpc::{Error, ErrorCode, Result};
use lsp_types::notification::*;
use lsp_types::request::*;
use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CodeAction, CodeActionParams, CodeActionResponse, CodeLens, CodeLensParams, ColorInformation,
    ColorPresentation, ColorPresentationParams, CompletionItem, CompletionParams,
    CompletionResponse, CreateFilesParams, DeleteFilesParams, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidChangeWorkspaceFoldersParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    DocumentColorParams, DocumentDiagnosticParams, DocumentDiagnosticReportResult,
    DocumentFormattingParams, DocumentHighlight, DocumentHighlightParams, DocumentLink,
    DocumentLinkParams, DocumentOnTypeFormattingParams, DocumentRangeFormattingParams,
    DocumentSymbolParams, DocumentSymbolResponse, ExecuteCommandParams, FoldingRange,
    FoldingRangeParams, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams,
    InitializeParams, InitializeResult, InitializedParams, InlayHint, InlayHintParams, InlineValue,
    InlineValueParams, LinkedEditingRangeParams, LinkedEditingRanges, Location, MessageType,
    Moniker, MonikerParams, PrepareRenameResponse, ReferenceParams, RenameFilesParams,
    RenameParams, SelectionRange, SelectionRangeParams, SemanticTokensDeltaParams,
    SemanticTokensFullDeltaResult, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult, SignatureHelp, SignatureHelpParams,
    SymbolInformation, TextDocumentPositionParams, TextEdit, TypeHierarchyItem,
    TypeHierarchyPrepareParams, TypeHierarchySubtypesParams, TypeHierarchySupertypesParams,
    WillSaveTextDocumentParams, WorkspaceDiagnosticParams, WorkspaceDiagnosticReportResult,
    WorkspaceEdit, WorkspaceSymbol, WorkspaceSymbolParams,
};
use serde_json::Value;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Dispatcher {
    handlers: Vec<Box<dyn Handler>>,
    connection: Connection,
    }

impl Dispatcher {
    pub fn new(connection: Connection) -> Self {
        Dispatcher {
            handlers: Vec::new(),
            connection,
                    }
    }

    pub fn setup<F>(&mut self, creators: Vec<F>)
    where
        F: FnOnce(Connection) -> Box<dyn Handler>,
    {
        for creator in creators {
            self.handlers.push(creator(self.connection.clone()));
        }
    }
}

#[wasm_bindgen]
impl Dispatcher {
    #[allow(unused_variables)]
    #[wasm_bindgen(js_class = Dispatcher, js_name = onRequest)]
    pub async fn on_request(
        &mut self,
        method: &str,
        params: JsValue,
    ) -> std::result::Result<JsValue, JsValue> {
        log(method);

        match method {
            Initialize::METHOD => {
                let params: InitializeParams = from_value(params).unwrap();
                let res = self.initialize(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            Shutdown::METHOD => {
                let params: InitializeParams = from_value(params).unwrap();
                let res = self.initialize(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            WillSaveWaitUntil::METHOD => {
                let params: WillSaveTextDocumentParams = from_value(params).unwrap();
                let res = self.will_save_wait_until(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            GotoDeclaration::METHOD => {
                let params: GotoDeclarationParams = from_value(params).unwrap();
                let res = self.goto_declaration(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            GotoDefinition::METHOD => {
                let params: GotoDefinitionParams = from_value(params).unwrap();
                let res = self.goto_definition(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            GotoTypeDefinition::METHOD => {
                let params: GotoTypeDefinitionParams = from_value(params).unwrap();
                let res = self.goto_type_definition(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            GotoImplementation::METHOD => {
                let params: GotoImplementationParams = from_value(params).unwrap();
                let res = self.goto_implementation(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            References::METHOD => {
                let params: ReferenceParams = from_value(params).unwrap();
                let res = self.references(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            CallHierarchyPrepare::METHOD => {
                let params: CallHierarchyPrepareParams = from_value(params).unwrap();
                let res = self.prepare_call_hierarchy(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();

                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            CallHierarchyIncomingCalls::METHOD => {
                let params: CallHierarchyIncomingCallsParams = from_value(params).unwrap();
                let res = self.incoming_calls(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            CallHierarchyOutgoingCalls::METHOD => {
                let params: CallHierarchyOutgoingCallsParams = from_value(params).unwrap();
                let res = self.outgoing_calls(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            TypeHierarchyPrepare::METHOD => {
                let params: TypeHierarchyPrepareParams = from_value(params).unwrap();
                let res = self.prepare_type_hierarchy(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            TypeHierarchySupertypes::METHOD => {
                let params: TypeHierarchySupertypesParams = from_value(params).unwrap();
                let res = self.supertypes(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            TypeHierarchySubtypes::METHOD => {
                let params: TypeHierarchySubtypesParams = from_value(params).unwrap();
                let res = self.subtypes(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            DocumentHighlightRequest::METHOD => {
                let params: DocumentHighlightParams = from_value(params).unwrap();
                let res = self.document_highlight(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            DocumentLinkRequest::METHOD => {
                let params: DocumentLinkParams = from_value(params).unwrap();
                let res = self.document_link(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            DocumentLinkResolve::METHOD => {
                let params: DocumentLink = from_value(params).unwrap();
                let res = self.document_link_resolve(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            HoverRequest::METHOD => {
                let params: HoverParams = from_value(params).unwrap();
                let res = self.hover(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            CodeLensRequest::METHOD => {
                let params: CodeLensParams = from_value(params).unwrap();
                let res = self.code_lens(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            CodeLensResolve::METHOD => {
                let params: CodeLens = from_value(params).unwrap();
                let res = self.code_lens_resolve(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            FoldingRangeRequest::METHOD => {
                let params: FoldingRangeParams = from_value(params).unwrap();
                let res = self.folding_range(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            SelectionRangeRequest::METHOD => {
                let params: SelectionRangeParams = from_value(params).unwrap();
                let res = self.selection_range(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };

                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            DocumentSymbolRequest::METHOD => {
                let params: DocumentSymbolParams = from_value(params).unwrap();
                let res = self.document_symbol(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            SemanticTokensFullRequest::METHOD => {
                let params: SemanticTokensParams = from_value(params).unwrap();
                let res = self.semantic_tokens_full(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            SemanticTokensFullDeltaRequest::METHOD => {
                let params: SemanticTokensDeltaParams = from_value(params).unwrap();
                let res = self.semantic_tokens_full_delta(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            SemanticTokensRangeRequest::METHOD => {
                let params: SemanticTokensRangeParams = from_value(params).unwrap();
                let res = self.semantic_tokens_range(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            InlineValueRequest::METHOD => {
                let params: InlineValueParams = from_value(params).unwrap();
                let res = self.inline_value(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            InlayHintRequest::METHOD => {
                let params: InlayHintParams = from_value(params).unwrap();
                let res = self.inlay_hint(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            InlayHintResolveRequest::METHOD => {
                let params: InlayHint = from_value(params).unwrap();
                let res = self.inlay_hint_resolve(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            MonikerRequest::METHOD => {
                let params: MonikerParams = from_value(params).unwrap();
                let res = self.moniker(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            Completion::METHOD => {
                let params: CompletionParams = from_value(params).unwrap();
                let res = self.completion(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            DocumentDiagnosticRequest::METHOD => {
                let params: DocumentDiagnosticParams = from_value(params).unwrap();
                let res = self.diagnostic(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            WorkspaceDiagnosticRequest::METHOD => {
                let params: WorkspaceDiagnosticParams = from_value(params).unwrap();
                let res = self.workspace_diagnostic(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            SignatureHelpRequest::METHOD => {
                let params: SignatureHelpParams = from_value(params).unwrap();
                let res = self.signature_help(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            CodeActionRequest::METHOD => {
                let params: CodeActionParams = from_value(params).unwrap();
                let res = self.code_action(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            CodeActionResolveRequest::METHOD => {
                let params: CodeAction = from_value(params).unwrap();
                let res = self.code_action_resolve(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            DocumentColor::METHOD => {
                let params: DocumentColorParams = from_value(params).unwrap();
                let res = self.document_color(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            ColorPresentationRequest::METHOD => {
                let params: ColorPresentationParams = from_value(params).unwrap();
                let res = self.color_presentation(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            Formatting::METHOD => {
                let params: DocumentFormattingParams = from_value(params).unwrap();
                let res = self.formatting(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            RangeFormatting::METHOD => {
                let params: DocumentRangeFormattingParams = from_value(params).unwrap();
                let res = self.range_formatting(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            OnTypeFormatting::METHOD => {
                let params: DocumentOnTypeFormattingParams = from_value(params).unwrap();
                let res = self.on_type_formatting(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            Rename::METHOD => {
                let params: RenameParams = from_value(params).unwrap();
                let res = self.rename(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            PrepareRenameRequest::METHOD => {
                let params: TextDocumentPositionParams = from_value(params).unwrap();
                let res = self.prepare_rename(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            LinkedEditingRange::METHOD => {
                let params: LinkedEditingRangeParams = from_value(params).unwrap();
                let res = self.linked_editing_range(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            WorkspaceSymbolRequest::METHOD => {
                let params: WorkspaceSymbolParams = from_value(params).unwrap();
                let res = self.symbol(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            WorkspaceSymbolResolve::METHOD => {
                let params: WorkspaceSymbol = from_value(params).unwrap();
                let res = self.symbol_resolve(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;

                Ok(result)
            }
            WillDeleteFiles::METHOD => {
                let params: DeleteFilesParams = from_value(params).unwrap();
                let res = self.will_delete_files(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            WillCreateFiles::METHOD => {
                let params: CreateFilesParams = from_value(params).unwrap();
                let res = self.will_create_files(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            WillRenameFiles::METHOD => {
                let params: RenameFilesParams = from_value(params).unwrap();
                let res = self.will_rename_files(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            ExecuteCommand::METHOD => {
                let params: ExecuteCommandParams = from_value(params).unwrap();
                let res = self.execute_command(params);
                let result = match res {
                    Ok(init) => {
                        let js = to_value(&init).unwrap();
                        js_sys::Promise::resolve(&js)
                    }
                    Err(err) => {
                        let js = to_value(&err).unwrap();
                        js_sys::Promise::reject(&js)
                    }
                };
                let result = wasm_bindgen_futures::JsFuture::from(result).await?;
                Ok(result)
            }
            _ => Err(JsValue::from_str("Method not dispatched")),
        }
    }

    #[allow(unused_variables)]
    #[wasm_bindgen(js_class = Dispatcher, js_name = onNotification)]
    pub async fn on_notification(
        &mut self,
        method: &str,
        params: JsValue,
    ) -> std::result::Result<(), JsValue> {
        log(method);

        match method {
            Initialized::METHOD => {
                let params: InitializedParams = from_value(params).unwrap();
                self.initialized(params);
                Ok(())
            }
            DidOpenTextDocument::METHOD => {
                let params: DidOpenTextDocumentParams = from_value(params).unwrap();
                self.did_open(params);
                Ok(())
            }
            DidChangeTextDocument::METHOD => {
                let params: DidChangeTextDocumentParams = from_value(params).unwrap();
                self.did_change(params);
                Ok(())
            }
            WillSaveTextDocument::METHOD => {
                let params: WillSaveTextDocumentParams = from_value(params).unwrap();
                self.will_save(params);
                Ok(())
            }
            DidSaveTextDocument::METHOD => {
                let params: DidSaveTextDocumentParams = from_value(params).unwrap();
                self.did_save(params);
                Ok(())
            }
            DidCloseTextDocument::METHOD => {
                let params: DidCloseTextDocumentParams = from_value(params).unwrap();
                self.did_close(params);
                Ok(())
            }
            DidChangeConfiguration::METHOD => {
                let params: DidChangeConfigurationParams = from_value(params).unwrap();
                self.did_change_configuration(params);
                Ok(())
            }
            DidChangeWatchedFiles::METHOD => {
                let params: DidChangeWatchedFilesParams = from_value(params).unwrap();
                self.did_change_watched_files(params);
                Ok(())
            }
            DidChangeWorkspaceFolders::METHOD => {
                let params: DidChangeWorkspaceFoldersParams = from_value(params).unwrap();
                self.did_change_workspace_folders(params);
                Ok(())
            }
            DidRenameFiles::METHOD => {
                let params: RenameFilesParams = from_value(params).unwrap();
                self.did_rename_files(params);
                Ok(())
            }
            DidCreateFiles::METHOD => {
                let params: CreateFilesParams = from_value(params).unwrap();
                self.did_create_files(params);
                Ok(())
            }
            DidDeleteFiles::METHOD => {
                let params: DeleteFilesParams = from_value(params).unwrap();
                self.did_delete_files(params);
                Ok(())
            }

            _ => Err(JsValue::from_str("Method not dispatched")),
        }
    }
}

impl Handler for Dispatcher {
    fn initialize(&mut self, params: InitializeParams) -> Result<InitializeResult> {
        let mut res = vec![];
        self.connection
            .log_message(MessageType::INFO, "Dispatcher initializing");
        for handler in &mut self.handlers {
            res.push(handler.initialize(params.clone()));
        }
        let res: Vec<InitializeResult> = res.iter().filter_map(|i| i.clone().ok()).collect();
        let mut result = InitializeResult::default();

        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.position_encoding.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.position_encoding =
                tmp_cap[0].capabilities.position_encoding.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.text_document_sync.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.text_document_sync =
                tmp_cap[0].capabilities.text_document_sync.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.selection_range_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.selection_range_provider =
                tmp_cap[0].capabilities.selection_range_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.hover_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.hover_provider = tmp_cap[0].capabilities.hover_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.completion_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.completion_provider =
                tmp_cap[0].capabilities.completion_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.signature_help_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.signature_help_provider =
                tmp_cap[0].capabilities.signature_help_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.definition_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.definition_provider =
                tmp_cap[0].capabilities.definition_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.type_definition_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.type_definition_provider =
                tmp_cap[0].capabilities.type_definition_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.implementation_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.implementation_provider =
                tmp_cap[0].capabilities.implementation_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.references_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.references_provider =
                tmp_cap[0].capabilities.references_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.document_highlight_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.document_highlight_provider =
                tmp_cap[0].capabilities.document_highlight_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.document_symbol_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.document_symbol_provider =
                tmp_cap[0].capabilities.document_symbol_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.workspace_symbol_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.workspace_symbol_provider =
                tmp_cap[0].capabilities.workspace_symbol_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.code_action_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.code_action_provider =
                tmp_cap[0].capabilities.code_action_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.code_lens_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.code_lens_provider = tmp_cap[0].capabilities.code_lens_provider;
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.document_formatting_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.document_formatting_provider =
                tmp_cap[0].capabilities.document_formatting_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.document_range_formatting_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.document_range_formatting_provider = tmp_cap[0]
                .capabilities
                .document_range_formatting_provider
                .clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| {
                i.capabilities
                    .document_on_type_formatting_provider
                    .is_some()
            })
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.document_on_type_formatting_provider = tmp_cap[0]
                .capabilities
                .document_on_type_formatting_provider
                .clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.rename_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.rename_provider = tmp_cap[0].capabilities.rename_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.document_link_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.document_link_provider =
                tmp_cap[0].capabilities.document_link_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.color_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.color_provider = tmp_cap[0].capabilities.color_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.folding_range_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.folding_range_provider =
                tmp_cap[0].capabilities.folding_range_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.declaration_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.declaration_provider =
                tmp_cap[0].capabilities.declaration_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.workspace.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.workspace = tmp_cap[0].capabilities.workspace.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.call_hierarchy_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.call_hierarchy_provider =
                tmp_cap[0].capabilities.call_hierarchy_provider;
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.semantic_tokens_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.semantic_tokens_provider =
                tmp_cap[0].capabilities.semantic_tokens_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.moniker_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.moniker_provider = tmp_cap[0].capabilities.moniker_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.linked_editing_range_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.linked_editing_range_provider = tmp_cap[0]
                .capabilities
                .linked_editing_range_provider
                .clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.inline_value_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.inline_value_provider =
                tmp_cap[0].capabilities.inline_value_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.inlay_hint_provider.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.inlay_hint_provider =
                tmp_cap[0].capabilities.inlay_hint_provider.clone();
        }
        let tmp_cap = res
            .iter()
            .filter(|i| i.capabilities.experimental.is_some())
            .collect::<Vec<&InitializeResult>>();
        if !tmp_cap.is_empty() {
            result.capabilities.experimental = tmp_cap[0].capabilities.experimental.clone();
        }
        Ok(result)
    }

    fn initialized(&mut self, params: InitializedParams) {
        self.connection
            .log_message(MessageType::INFO, "Dispatcher initialized");
        for handler in &mut self.handlers {
            handler.initialized(params);
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        for handler in &mut self.handlers {
            let _ = handler.shutdown();
        }
        Ok(())
    }

    fn did_open(&mut self, params: DidOpenTextDocumentParams) {
        for handler in &mut self.handlers {
            handler.did_open(params.clone());
        }
    }

    fn did_change(&mut self, params: DidChangeTextDocumentParams) {
        for handler in &mut self.handlers {
            handler.did_change(params.clone());
        }
    }

    fn will_save(&mut self, params: WillSaveTextDocumentParams) {
        for handler in &mut self.handlers {
            handler.will_save(params.clone());
        }
    }

    fn will_save_wait_until(
        &mut self,
        params: WillSaveTextDocumentParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let mut text_edit: Vec<TextEdit> = vec![];
        for handler in &mut self.handlers {
            let res = handler.will_save_wait_until(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn did_save(&mut self, params: DidSaveTextDocumentParams) {
        for handler in &mut self.handlers {
            handler.did_save(params.clone());
        }
    }

    fn did_close(&mut self, params: DidCloseTextDocumentParams) {
        for handler in &mut self.handlers {
            handler.did_close(params.clone());
        }
    }

    fn goto_declaration(
        &mut self,
        params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>> {
        for handler in &mut self.handlers {
            let res = handler.goto_declaration(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn goto_definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        for handler in &mut self.handlers {
            let res = handler.goto_definition(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn goto_type_definition(
        &mut self,
        params: GotoTypeDefinitionParams,
    ) -> Result<Option<GotoTypeDefinitionResponse>> {
        for handler in &mut self.handlers {
            let res = handler.goto_type_definition(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn goto_implementation(
        &mut self,
        params: GotoImplementationParams,
    ) -> Result<Option<GotoImplementationResponse>> {
        for handler in &mut self.handlers {
            let res = handler.goto_implementation(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn references(&mut self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let mut text_edit: Vec<Location> = vec![];
        for handler in &mut self.handlers {
            let res = handler.references(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn prepare_call_hierarchy(
        &mut self,
        params: CallHierarchyPrepareParams,
    ) -> Result<Option<Vec<CallHierarchyItem>>> {
        let mut text_edit: Vec<CallHierarchyItem> = vec![];
        for handler in &mut self.handlers {
            let res = handler.prepare_call_hierarchy(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn incoming_calls(
        &mut self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        let mut text_edit: Vec<CallHierarchyIncomingCall> = vec![];
        for handler in &mut self.handlers {
            let res = handler.incoming_calls(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn outgoing_calls(
        &mut self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        let mut text_edit: Vec<CallHierarchyOutgoingCall> = vec![];
        for handler in &mut self.handlers {
            let res = handler.outgoing_calls(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn prepare_type_hierarchy(
        &mut self,
        params: TypeHierarchyPrepareParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let mut text_edit: Vec<TypeHierarchyItem> = vec![];
        for handler in &mut self.handlers {
            let res = handler.prepare_type_hierarchy(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn supertypes(
        &mut self,
        params: TypeHierarchySupertypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let mut text_edit: Vec<TypeHierarchyItem> = vec![];
        for handler in &mut self.handlers {
            let res = handler.supertypes(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn subtypes(
        &mut self,
        params: TypeHierarchySubtypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let mut text_edit: Vec<TypeHierarchyItem> = vec![];
        for handler in &mut self.handlers {
            let res = handler.subtypes(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn document_highlight(
        &mut self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        let mut text_edit: Vec<DocumentHighlight> = vec![];
        for handler in &mut self.handlers {
            let res = handler.document_highlight(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn document_link(&mut self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        let mut text_edit: Vec<DocumentLink> = vec![];
        for handler in &mut self.handlers {
            let res = handler.document_link(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn document_link_resolve(&mut self, params: DocumentLink) -> Result<DocumentLink> {
        for handler in &mut self.handlers {
            let res = handler.document_link_resolve(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn hover(&mut self, params: HoverParams) -> Result<Option<Hover>> {
        for handler in &mut self.handlers {
            let res = handler.hover(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn code_lens(&mut self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let mut text_edit: Vec<CodeLens> = vec![];
        for handler in &mut self.handlers {
            let res = handler.code_lens(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn code_lens_resolve(&mut self, params: CodeLens) -> Result<CodeLens> {
        for handler in &mut self.handlers {
            let res = handler.code_lens_resolve(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn folding_range(&mut self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let mut text_edit: Vec<FoldingRange> = vec![];
        for handler in &mut self.handlers {
            let res = handler.folding_range(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn selection_range(&mut self, params: SelectionRangeParams) -> Result<Option<Vec<SelectionRange>>> {
        let mut text_edit: Vec<SelectionRange> = vec![];
        for handler in &mut self.handlers {
            let res = handler.selection_range(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn document_symbol(
        &mut self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        for handler in &mut self.handlers {
            let res = handler.document_symbol(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn semantic_tokens_full(
        &mut self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        for handler in &mut self.handlers {
            let res = handler.semantic_tokens_full(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn semantic_tokens_full_delta(
        &mut self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        for handler in &mut self.handlers {
            let res = handler.semantic_tokens_full_delta(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn semantic_tokens_range(
        &mut self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        for handler in &mut self.handlers {
            let res = handler.semantic_tokens_range(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn inline_value(&mut self, params: InlineValueParams) -> Result<Option<Vec<InlineValue>>> {
        let mut text_edit: Vec<InlineValue> = vec![];
        for handler in &mut self.handlers {
            let res = handler.inline_value(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn inlay_hint(&mut self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let mut text_edit: Vec<InlayHint> = vec![];
        for handler in &mut self.handlers {
            let res = handler.inlay_hint(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn inlay_hint_resolve(&mut self, params: InlayHint) -> Result<InlayHint> {
        for handler in &mut self.handlers {
            let res = handler.inlay_hint_resolve(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn moniker(&mut self, params: MonikerParams) -> Result<Option<Vec<Moniker>>> {
        let mut text_edit: Vec<Moniker> = vec![];
        for handler in &mut self.handlers {
            let res = handler.moniker(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn completion(&mut self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        for handler in &mut self.handlers {
            let res = handler.completion(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn completion_resolve(&mut self, params: CompletionItem) -> Result<CompletionItem> {
        for handler in &mut self.handlers {
            let res = handler.completion_resolve(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn diagnostic(
        &mut self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        for handler in &mut self.handlers {
            let res = handler.diagnostic(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn workspace_diagnostic(
        &mut self,
        params: WorkspaceDiagnosticParams,
    ) -> Result<WorkspaceDiagnosticReportResult> {
        for handler in &mut self.handlers {
            let res = handler.workspace_diagnostic(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn signature_help(&mut self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        for handler in &mut self.handlers {
            let res = handler.signature_help(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn code_action(&mut self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        for handler in &mut self.handlers {
            let res = handler.code_action(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn code_action_resolve(&mut self, params: CodeAction) -> Result<CodeAction> {
        for handler in &mut self.handlers {
            let res = handler.code_action_resolve(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn document_color(&mut self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        let mut text_edit: Vec<ColorInformation> = vec![];
        for handler in &mut self.handlers {
            let res = handler.document_color(params.clone());
            if let Ok(val) = res {
                text_edit.extend(val);
            }
        }
        Ok(text_edit)
    }

    fn color_presentation(
        &mut self,
        params: ColorPresentationParams,
    ) -> Result<Vec<ColorPresentation>> {
        let mut text_edit: Vec<ColorPresentation> = vec![];
        for handler in &mut self.handlers {
            let res = handler.color_presentation(params.clone());
            if let Ok(val) = res {
                text_edit.extend(val);
            }
        }
        Ok(text_edit)
    }

    fn formatting(&mut self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let mut text_edit: Vec<TextEdit> = vec![];
        for handler in &mut self.handlers {
            let res = handler.formatting(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn range_formatting(
        &mut self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let mut text_edit: Vec<TextEdit> = vec![];
        for handler in &mut self.handlers {
            let res = handler.range_formatting(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn on_type_formatting(
        &mut self,
        params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let mut text_edit: Vec<TextEdit> = vec![];
        for handler in &mut self.handlers {
            let res = handler.on_type_formatting(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn rename(&mut self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        for handler in &mut self.handlers {
            let res = handler.rename(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn prepare_rename(
        &mut self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        for handler in &mut self.handlers {
            let res = handler.prepare_rename(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn linked_editing_range(
        &mut self,
        params: LinkedEditingRangeParams,
    ) -> Result<Option<LinkedEditingRanges>> {
        for handler in &mut self.handlers {
            let res = handler.linked_editing_range(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn symbol(&mut self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        let mut text_edit: Vec<SymbolInformation> = vec![];
        for handler in &mut self.handlers {
            let res = handler.symbol(params.clone());
            if let Ok(Some(val)) = res {
                text_edit.extend(val);
            }
        }
        Ok(Some(text_edit))
    }

    fn symbol_resolve(&mut self, params: WorkspaceSymbol) -> Result<WorkspaceSymbol> {
        for handler in &mut self.handlers {
            let res = handler.symbol_resolve(params.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(Error::new(ErrorCode::MethodNotFound))
    }

    fn did_change_configuration(&mut self, params: DidChangeConfigurationParams) {
        for handler in &mut self.handlers {
            handler.did_change_configuration(params.clone());
        }
    }

    fn did_change_workspace_folders(&mut self, params: DidChangeWorkspaceFoldersParams) {
        for handler in &mut self.handlers {
            handler.did_change_workspace_folders(params.clone());
        }
    }

    fn will_create_files(&mut self, params: CreateFilesParams) -> Result<Option<WorkspaceEdit>> {
        for handler in &mut self.handlers {
            let res = handler.will_create_files(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn did_create_files(&mut self, params: CreateFilesParams) {
        for handler in &mut self.handlers {
            handler.did_create_files(params.clone());
        }
    }

    fn will_rename_files(&mut self, params: RenameFilesParams) -> Result<Option<WorkspaceEdit>> {
        for handler in &mut self.handlers {
            let res = handler.will_rename_files(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn did_rename_files(&mut self, params: RenameFilesParams) {
        for handler in &mut self.handlers {
            handler.did_rename_files(params.clone());
        }
    }

    fn will_delete_files(&mut self, params: DeleteFilesParams) -> Result<Option<WorkspaceEdit>> {
        for handler in &mut self.handlers {
            let res = handler.will_delete_files(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }

    fn did_delete_files(&mut self, params: DeleteFilesParams) {
        for handler in &mut self.handlers {
            handler.did_delete_files(params.clone());
        }
    }

    fn did_change_watched_files(&mut self, params: DidChangeWatchedFilesParams) {
        for handler in &mut self.handlers {
            handler.did_change_watched_files(params.clone());
        }
    }

    fn execute_command(&mut self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        for handler in &mut self.handlers {
            let res = handler.execute_command(params.clone());
            if let Ok(Some(_)) = res {
                return res;
            }
        }
        Ok(None)
    }
}
