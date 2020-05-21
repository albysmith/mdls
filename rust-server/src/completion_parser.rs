use log::info;
use lsp_types::{CompletionItem, CompletionParams};
use roxmltree::*;
use std::fs;
pub fn simple_complete(params: CompletionParams) -> Vec<CompletionItem> {
    let mut namespace = vec![];
    let error_string = format!(
        "uri conversion is fucked {:#?}",
        params
            .text_document_position
            .text_document
            .uri
            .to_file_path()
            .unwrap()
    );

    let string = fs::read_to_string(
        params
            .text_document_position
            .text_document
            .uri
            .to_file_path()
            .unwrap(),
    )
    .unwrap();
    let doc = roxmltree::Document::parse(&string).expect("file isnt xml");
    for node in doc.descendants() {
        if node.tag_name().name() == "cue" {
            if let Some(n) = node.attribute("namespace") {
                if n == "this" {
                    namespace.clear()
                }
            }
        } else {
            if let Some(n) = node.parent() {
                if n.tag_name().name() == "actions" {
                    if let Some(attr) = node.attribute("name") {
                        let item = CompletionItem {
                            label: attr.to_owned(),
                            ..CompletionItem::default()
                        };
                        if !namespace.contains(&item) {
                            namespace.push(item);
                        }
                    }
                }
            }
        }
        let pos = doc.text_pos_at(node.range().start);
        if pos.row as u64 == params.text_document_position.position.line {
            break;
        }
    }
    namespace
}
pub fn parent_complete(params: CompletionParams) -> Vec<CompletionItem> {
    let mut namespace = vec![];
    let error_string = format!(
        "uri conversion is fucked {:#?}",
        params
            .text_document_position
            .text_document
            .uri
            .to_file_path()
            .unwrap()
    );

    let string = fs::read_to_string(
        params
            .text_document_position
            .text_document
            .uri
            .to_file_path()
            .unwrap(),
    )
    .unwrap();
    let doc = roxmltree::Document::parse(&string).expect("file isnt xml");

    namespace
}
