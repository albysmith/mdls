// use log::info;
use lsp_server::{RequestId, Response};
use lsp_types::{Hover, HoverParams, MarkedString};
use std::fs;

use crate::scriptproperties::*;

pub fn get_hover_resp(id: RequestId, params: HoverParams, scriptps: &ScriptProperties) -> Response {
    // info!("called hover function");

    // get &str for what I'm hovering on
    if let Some((_byte_position, string)) = get_node_and_string(&params) {
        // let file_uri = params.text_document_position_params.text_document.uri;
        let line = string
            .lines()
            .nth(params.text_document_position_params.position.line as usize);
        let character = params
            .text_document_position_params
            .position
            .character
            .clone() as usize;
        if let Ok(line_doc) = roxmltree::Document::parse(&line.unwrap()) {
            let node = line_doc.root().first_child().unwrap();
            // info!("HOVER line_doc node: {:?}", node.tag_name());
            for attr in node.attributes() {
                if attr.value_range().end > character && attr.value_range().start < character {
                    // now we have the attribute
                    let char_into_attr = character - attr.value_range().start;
                    let mut target = String::new();
                    let mut flag = false;
                    for (i, character) in attr.value().chars().enumerate() {
                        if i == char_into_attr {
                            flag = true
                        }
                        match character {
                            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k'
                            | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v'
                            | 'w' | 'x' | 'y' | 'z' | '$' => {
                                target.push(character);
                            }
                            _ => {
                                if flag == true {
                                    break;
                                } else {
                                    target.clear()
                                }
                            }
                        }
                    }
                    // do something with target here
                    // check against scriptproperties
                    let find = scriptps.search(&target);

                    // make everything the right type
                    let mut hovers = vec![];
                    for entry in find.iter() {
                        hovers.push(MarkedString::String(format!(
                            "Datatype: {:?} Property: {} Result: {} Type: {}",
                            entry.datatype,
                            if let Some(name) = &entry.prop_name {
                                name
                            } else {
                                "unknown"
                            },
                            if let Some(name) = &entry.prop_result {
                                name
                            } else {
                                "unknown"
                            },
                            if let Some(name) = &entry.prop_type {
                                name
                            } else {
                                "unknown"
                            }
                        )))
                    }

                    let result = Some(lsp_types::Hover {
                        contents: lsp_types::HoverContents::Array(hovers),
                        range: None,
                    });

                    let result = serde_json::to_value(&result).unwrap();
                    let resp = Response {
                        id,
                        result: Some(result),
                        error: None,
                    };
                    return resp;
                }
            }
        }
    }
    let result: Option<Hover> = None;
    let result = serde_json::to_value(&result).unwrap();
    let resp = Response {
        id,
        result: Some(result),
        error: None,
    };
    return resp;
}

fn get_node_and_string(params: &HoverParams) -> Option<(usize, String)> {
    if let Ok(path) = params
        .text_document_position_params
        .text_document
        .uri
        .to_file_path()
    {
        if let Ok(string) = fs::read_to_string(path) {
            if let Some(line) = string
                .lines()
                .nth(params.text_document_position_params.position.line as usize)
            {
                if let Some(byte_position) = string.find(line) {
                    Some((byte_position, string))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
