use specs::prelude::*;
use std::error::Error;

use log::info;
// use lsp_types::notification::DidChangeTextDocument;
use lsp_types::request::*;
use lsp_types::*;
// use lsp_types::{
//     CodeActionOptions, CodeActionProviderCapability, CodeLensOptions, CompletionItem,
//     CompletionOptions, CompletionResponse, DocumentOnTypeFormattingOptions,
//     FoldingRangeProviderCapability, ImplementationProviderCapability, InitializeParams,
//     RenameOptions, RenameProviderCapability, SaveOptions, SelectionRangeProviderCapability,
//     ServerCapabilities, SignatureHelpOptions, TextDocumentSyncCapability, TextDocumentSyncKind,
//     TextDocumentSyncOptions, TypeDefinitionProviderCapability, WorkDoneProgressOptions,
// };

use lsp_server::{Connection, Message, Request, RequestId, Response};

#[macro_use]
mod macros;
use macros::*;

mod completion_parser;
use completion_parser::*;

mod definition_parser;
use definition_parser::*;

mod type_checker;
use type_checker::*;

mod type_annotations;
use type_annotations::*;

mod hover;
use hover::*;

mod expression_parser;
use expression_parser::*;

mod scriptproperties;
use scriptproperties::*;

mod data_store;
use data_store::*;

mod world_trigger;
use world_trigger::*;

mod error_handling;
use error_handling::*;

mod tests;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Set up logging. Because `stdio_transport` gets a lock on stdout and stdin, we must have
    // our logging only write out to stderr.
    flexi_logger::Logger::with_str("info").start().unwrap();
    info!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_settings = ServerCapabilities {
        // text_document_sync: Some(TextDocumentSyncCapability::Options(
        //     TextDocumentSyncOptions {
        //         open_close: Some(true),
        //         change: Some(TextDocumentSyncKind::Full),
        //         will_save: None,
        //         will_save_wait_until: None,
        //         save: Some(SaveOptions::default()),
        //     },
        // )),
        hover_provider: Some(true),
        completion_provider: Some(CompletionOptions {
            resolve_provider: None,
            trigger_characters: Some(vec![":".to_string(), ".".to_string()]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        declaration_provider: None,
        definition_provider: Some(true),
        type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
        implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
        references_provider: Some(true),
        document_highlight_provider: Some(true),
        document_symbol_provider: Some(true),
        workspace_symbol_provider: Some(true),
        code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
            // Advertise support for all built-in CodeActionKinds
            code_action_kinds: Some(vec![
                lsp_types::code_action_kind::EMPTY.to_string(),
                lsp_types::code_action_kind::QUICKFIX.to_string(),
                lsp_types::code_action_kind::REFACTOR.to_string(),
                lsp_types::code_action_kind::REFACTOR_EXTRACT.to_string(),
                lsp_types::code_action_kind::REFACTOR_INLINE.to_string(),
                lsp_types::code_action_kind::REFACTOR_REWRITE.to_string(),
                lsp_types::code_action_kind::SOURCE.to_string(),
                lsp_types::code_action_kind::SOURCE_ORGANIZE_IMPORTS.to_string(),
            ]),
            work_done_progress_options: Default::default(),
        })),
        code_lens_provider: Some(CodeLensOptions {
            resolve_provider: Some(true),
        }),
        document_formatting_provider: Some(true),
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
            first_trigger_character: "=".to_string(),
            more_trigger_character: Some(vec![".".to_string(), ">".to_string()]),
        }),
        selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        // semantic_highlighting: None,
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        rename_provider: Some(RenameProviderCapability::Options(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        document_link_provider: None,
        color_provider: None,
        execute_command_provider: None,
        workspace: None,
        // call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
        // semantic_tokens_provider: Some(
        //     SemanticTokensOptions {
        //         legend: SemanticTokensLegend {
        //             token_types: semantic_tokens::SUPPORTED_TYPES.to_vec(),
        //             token_modifiers: semantic_tokens::SUPPORTED_MODIFIERS.to_vec(),
        //         },

        //         document_provider: Some(SemanticTokensDocumentProvider::Bool(true)),
        //         range_provider: Some(true),
        //         work_done_progress_options: Default::default(),
        //     }
        //     .into(),
        // ),
        experimental: Default::default(),
        ..ServerCapabilities::default()
    };
    let server_capabilities = serde_json::to_value(&server_settings).unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(&connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    info!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: &Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    info!("starting example main loop");

    // make our entity component system here
    let mut ecs = parse_file(_params.root_uri.clone());

    // this one uses world_trigger
    let mut ecs2 = parse_workspace(_params.root_uri.clone());
    // this is the simpler non-parallel version of calling a system to run  
    // see data_store for the PrintMe function if you want to use the dispatcher
    let mut hello_world = PrintNames;
    hello_world.run_now(&ecs2);
    ecs2.maintain();

    // also bring over our scriptproperties
    let scriptps = ScriptProperties::new(include_str!("reference/scriptproperties.xml"));

    for msg in &connection.receiver {
        // info!("got msg: {:?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                // info!("got request: {:?}", req);

                let mut request = ReqMessage { req: req };
                if let Ok((id, params)) = request.cast::<HoverRequest>() {
                    // info!("got Hover request #{}: {:?}", id, params);
                    let resp = get_hover_resp(id, params, &scriptps);
                    connection.sender.send(Message::Response(resp))?;
                    continue;
                }
                if let Ok((id, params)) = request.cast::<Completion>() {
                    // info!("got COMPLETION request!!! {:#?}", params);

                    let result = CompletionResponse::Array(simple_complete(params));
                    let result = serde_json::to_value(&result).unwrap();
                    let resp = Response {
                        id,
                        result: Some(result),
                        error: None,
                    };
                    connection.sender.send(Message::Response(resp))?;
                    continue;
                }
                if let Ok((id, params)) = request.cast::<GotoDefinition>() {
                    // info!("got gotoDefinition request #{}: {:?}", id, params);
                    let result = Some(lsp_types::GotoDefinitionResponse::Array(simple_definition(
                        params, &mut ecs,
                    )));
                    let result = serde_json::to_value(&result).unwrap();
                    let resp = Response {
                        id,
                        result: Some(result),
                        error: None,
                    };
                    connection.sender.send(Message::Response(resp))?;
                    continue;
                }
                // match cast::<GotoDefinition>(req) {
                //     Ok((id, params)) => {
                // info!("got gotoDefinition request #{}: {:?}", id, params);
                //         let result = Some(lsp_types::GotoDefinitionResponse::Array(Vec::new()));
                //         let result = serde_json::to_value(&result).unwrap();
                //         let resp = Response {
                //             id,
                //             result: Some(result),
                //             error: None,
                //         };
                //         connection.sender.send(Message::Response(resp))?;
                //         continue;
                //     }
                //     Err(req) => req,
                // };
                // match cast::<HoverRequest>(req) {
                //     Ok((id, params)) => {
                //         info!("got Hover request #{}: {:?}", id, params);
                //         let result = Some(lsp_types::Hover {
                //             contents: lsp_types::HoverContents::Scalar(
                //                 lsp_types::MarkedString::String("Hello World!".to_string()),
                //             ),
                //             range: None,
                //         });
                //         let result = serde_json::to_value(&result).unwrap();
                //         let resp = Response {
                //             id,
                //             result: Some(result),
                //             error: None,
                //         };
                //         connection.sender.send(Message::Response(resp))?;
                //         continue;
                //     }
                //     Err(req) => (),
                // }
                // ...
            }
            Message::Response(resp) => {
                // info!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                // info!("got notification: {:?}", not);
            }
        }
    }
    Ok(())
}

// fn cast<R>(req: Request) -> Result<(RequestId, R::Params), Request>
// where
//     R: lsp_types::request::Request,
//     R::Params: serde::de::DeserializeOwned,
// {
//     req.extract(R::METHOD)
// }

#[derive(Clone)]
struct ReqMessage {
    req: Request,
}

impl ReqMessage {
    fn cast<R>(&mut self) -> Result<(RequestId, R::Params), Request>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
    {
        self.clone().req.extract(R::METHOD)
    }
}
