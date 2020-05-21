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
    let line = string
        .lines()
        .nth(params.text_document_position.position.line as usize + 1)
        .expect("line iterator died");
    let byte_position = string.find(line).expect("ok this line should be here");
    info!("byte position{}", byte_position);
    let doc = roxmltree::Document::parse(&string).expect("file isnt xml");
    for (i, node) in doc.descendants().enumerate() {
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
        if (node.tag_name().name() != "mdscript")
            & (node.tag_name().name() != "cues")
            & (node.tag_name().name() != "cue")
            & (node.tag_name().name() != "actions")
            & (node.tag_name().name() != "")
        {
            info!(
                "node.range():{:?} name: {:?}",
                node.range(),
                node.tag_name().name()
            );
            if node.range().end >= byte_position {
                info!("loop broken ");
                break;
            }
        }

        // let pos = doc.text_pos_at(node.range().starat);
        // if pos.row as u64 == params.text_document_position.position.line {
        // }
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
