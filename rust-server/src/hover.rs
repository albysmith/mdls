// use log::info;
use lsp_server::{RequestId, Response};
use lsp_types::{Hover, HoverParams, MarkedString};
use std::fs;
use std::path::PathBuf;

use crate::*;

pub fn get_hover_resp(id: RequestId, params: HoverParams, scriptps: &ScriptProperties) -> Response {
    // info!("called hover function");

    // get &str for what I'm hovering on
    if let Some((string, path)) = get_string_and_path(&params) {
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

fn get_string_and_path(params: &HoverParams) -> Option<(String, PathBuf)> {
    if let Ok(path) = params
        .text_document_position_params
        .text_document
        .uri
        .to_file_path()
    {
        if let Ok(string) = fs::read_to_string(&path) {
            Some((string, path))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn new_hover_resp(id: RequestId, params: HoverParams, world: &mut World) -> Response {
    let result = Some(lsp_types::Hover {
        contents: lsp_types::HoverContents::Array(get_hover_values(params, world)),
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

fn get_hover_values(params: HoverParams, world: &mut World) -> Vec<MarkedString> {
    let mut type_vec = vec![];
    if let Some((string, path)) = get_string_and_path(&params) {
        let mut byte_number = 0;
        let hover_character = params
            .text_document_position_params
            .position
            .character
            .clone() as usize;
        let hover_line = params.text_document_position_params.position.line.clone() as usize;




        for (i,line) in string.lines().enumerate() {
            if i == hover_line {
                byte_number += hover_character;
                let variable_storage = world.read_storage::<components::Variable>();
                let span_storage = world.read_storage::<Span>();
                let node_storage = world.read_storage::<components::Node>();
                let entities = world.entities();
                for (span, var, entity) in (&span_storage, &variable_storage, &entities).join() {
                    if PathBuf::from(&var.path) == path {
                        // info!(
                        //     "bp: {:?} start: {:?} end: {:?}",
                        //     byte_number, span.start, span.end
                        // );
                        if byte_number > span.start && byte_number < span.end {
                            info!("byte_number: {:?}", byte_number);
                            if let Some(node) = node_storage.get(var.node.unwrap()) {
                                info!("node: {:?}", node);
                                if let Some(method) = &node.method {
                                    info!("method: {:?}", method);
                                    for output in method.output.iter() {
                                        info!("output: {:?}", output);
                                        info!("output.attr: {:?} var.name: {:?}", output.attr, var.name);
                                        if output.attr == var.name {
                                            info!("matched");
                                            if let Some(types) = &output.contains {
                                                info!("types: {:?}", types);
                                                for value in types {
                                                    info!("value: {:?}", value);
                                                    type_vec.push(MarkedString::String(format!("{:?}",value)))
                                                }
                                            }
                                        }
                                    }
                                } else if let Some(event) = &node.event {

                                }
                            }
                            break;
                        }
                    }
                }
            } else if i == 0 {
                byte_number += line.len();
            } else {
                byte_number += line.len() + 2;
            }

            // now we have the attribute
        }
    }
    type_vec
}

// struct TypeInference;
// impl<'a> System<'a> for TypeInference {
//     type SystemData = (
//         Entities<'a>,
//         WriteStorage<'a, components::Variable>,
//         ReadStorage<'a, Span>,
//         ReadStorage<'a, components::Node>,
//         Read<'a, EventList>,
//         Read<'a, MethodList>,
//     );
//     fn run(&mut self, (entities, mut variable_storage, span_storage, node_storage, eventlist): Self::SystemData) {
//         for (entity, span) in (&entities, &span_storage).join() {

//         }
//     }

// }
