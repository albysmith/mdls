use lsp_server::{RequestId, Response};
use lsp_types::{Hover, HoverParams, MarkedString};
use std::fs;
use std::path::PathBuf;

use crate::*;

pub fn get_hover_resp(id: RequestId, params: HoverParams, scriptps: &ScriptProperties) -> Response {
    if let Some((string, path)) = get_string_and_path(&params) {
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
            for attr in node.attributes() {
                if attr.value_range().end > character && attr.value_range().start < character {
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
                    let find = scriptps.search(&target);
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

pub fn new_hover_resp(id: RequestId, params: HoverParams, world: &World) -> Response {
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

fn get_hover_values(params: HoverParams, world: &World) -> Vec<MarkedString> {
    let mut type_vec = vec![];
    if let Some((string, path)) = get_string_and_path(&params) {
        let mut byte_number = 0;
        let hover_character = params
            .text_document_position_params
            .position
            .character
            .clone() as usize;
        let hover_line = params.text_document_position_params.position.line.clone() as usize;

        for (i, line) in string.lines().enumerate() {
            if i == hover_line {
                byte_number += hover_character;
                let variable_storage = world.read_storage::<components::Variable>();
                let span_storage = world.read_storage::<Span>();
                let entities = world.entities();
                for (span, var, entity) in (&span_storage, &variable_storage, &entities).join() {
                    if PathBuf::from(&var.path) == path {
                        if byte_number > span.start && byte_number < span.end {
                            if let Some(data_types) = get_types(var, world) {
                                for d in data_types.iter() {
                                    let text = format!("{:?}", d).to_ascii_lowercase();
                                    type_vec.push(MarkedString::String(format!(
                                        "possible type: *{}*",
                                        text
                                    )));
                                }
                            } else {
                                let char_into_attr = byte_number - span.start;
                                let mut target = String::new();
                                let mut flag = false;
                                for (i, character) in var.value.chars().enumerate() {
                                    if i == char_into_attr {
                                        flag = true
                                    }
                                    match character.to_ascii_lowercase() {
                                        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i'
                                        | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r'
                                        | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | '$'
                                        | '_' => {
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
                                info!("{}", target);
                                if !target.contains("$") {
                                    let scriptps = world.read_resource::<ScriptProperties>();
                                    let find = scriptps.search(&target);
                                    for entry in find.iter() {
                                        let text =
                                            format!("{:?}", entry.datatype).to_ascii_lowercase();
                                        type_vec.push(MarkedString::String(format!(
                                            "property of type: *{}* | **result type: {}** | description: {}",
                                            text,
                                            if let Some(name) = &entry.prop_type {
                                                name
                                            } else {
                                                "unknown"
                                            },
                                            if let Some(name) = &entry.prop_result {
                                                name
                                            } else {
                                                "unknown"
                                            },
                                        )))
                                    }
                                }
                                let mut namespace_cues = vec![];
                                get_namespace_cues(&mut namespace_cues, world, var.cue);

                                for n_cue in namespace_cues.into_iter() {
                                    for cue_var in n_cue.variables.iter() {
                                        let cvariable = cue_var.to_owned();
                                        if cvariable != entity {
                                            if let Some(var_comp) = variable_storage.get(cvariable)
                                            {
                                                if var_comp.value == var.value
                                                    || (target == var_comp.value
                                                        && target.contains("$"))
                                                {
                                                    if let Some(d_types) =
                                                        get_types(var_comp, world)
                                                    {
                                                        for d in d_types.iter() {
                                                            let text = format!("{:?}", d)
                                                                .to_ascii_lowercase();
                                                            type_vec.push(MarkedString::String(
                                                                format!(
                                                                    "possible type: *{}*",
                                                                    text
                                                                ),
                                                            ));
                                                        }
                                                    }
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            } else {
                byte_number += line.len() + 2;
            }
        }
    }
    type_vec
}

fn get_types(var: &components::Variable, world: &World) -> Option<Vec<Datatypes>> {
    let node_storage = world.read_storage::<components::Node>();
    let variable_storage = world.read_storage::<components::Variable>();

    if let Some(node) = node_storage.get(var.node.unwrap()) {
        if let Some(method) = &node.method {
            for output in method.output.iter() {
                if output.attr == var.name {
                    let mut multiple = false;
                    for attribute in node.variables.iter() {
                        if let Some(comp) = variable_storage.get(attribute.to_owned()) {
                            if comp.name == "multiple".to_string() {
                                if comp.value == "true".to_string() {
                                    multiple = true
                                }
                            }
                        }
                    }
                    if multiple == true {
                        if let Some(types) = &output.datatype {
                            let mut type_vec = vec![];
                            for value in types {
                                type_vec.push(value.to_owned());
                            }
                            return Some(type_vec);
                        }
                    }
                    if let Some(types) = &output.contains {
                        let mut type_vec = vec![];
                        for value in types {
                            type_vec.push(value.to_owned());
                        }
                        return Some(type_vec);
                    } else if let Some(types) = &output.datatype {
                        let mut type_vec = vec![];
                        for value in types {
                            type_vec.push(value.to_owned());
                        }
                        return Some(type_vec);
                    }
                }
            }
        }
    }
    None
}

pub fn get_namespace_cues(cue_vec: &mut Vec<components::Cue>, world: &World, op_cue: Option<Entity>) {
    let cue_storage = world.read_storage::<components::Cue>();
    if let Some(e) = op_cue {
        if let Some(cue) = cue_storage.get(e) {
            cue_vec.push(cue.to_owned());
            get_namespace_cues(cue_vec, world, cue.parent)
        }
    }
}

pub fn new_get_hover_values(params: HoverParams, world: &World) -> Vec<MarkedString> {
    let mut type_vec = vec![];
    if let Some((string, path)) = get_string_and_path(&params) {
        let mut byte_number = 0;
        let hover_character = params
            .text_document_position_params
            .position
            .character
            .clone() as usize;
        let hover_line = params.text_document_position_params.position.line.clone() as usize;
        for (i, line) in string.lines().enumerate() {
            if i == hover_line {
                byte_number += hover_character;
                let variable_storage = world.read_storage::<components::Variable>();
                let span_storage = world.read_storage::<Span>();
                let entities = world.entities();
                for (span, var, entity) in (&span_storage, &variable_storage, &entities).join() {
                    if PathBuf::from(&var.path) == path {
                        if byte_number > span.start && byte_number < span.end {
                            let expression_chain =
                                expression_parser::parse_expression(var.value.to_owned());
                            
                            break;
                        }
                    }
                }
            }
        }
    }

    type_vec
}
