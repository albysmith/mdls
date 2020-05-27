use crate::data_store::*;
use log::info;
use lsp_types::{GotoDefinitionParams, Location, Position, Range, Url};
use specs::prelude::*;
use std::fs;
use std::path::PathBuf;

pub fn simple_definition(params: GotoDefinitionParams, world: &mut World) -> Vec<Location> {
    info!("called simple_definition function");
    let mut locations = vec![];
    if let Some((_byte_position, string, file_path)) = get_node_and_string(&params) {
        let file_uri = params.text_document_position_params.text_document.uri;
        if let Ok(doc) = roxmltree::Document::parse(&string) {
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
                info!("line_doc node: {:?}", node.tag_name());
                for attr in node.attributes() {
                    if attr.value_range().end > character && attr.value_range().start < character {
                        info!(
                            "found correct value: {} character {}, start {}, end {}",
                            attr.value(),
                            character,
                            attr.value_range().start,
                            attr.value_range().end
                        );
                        let char_into_attr = character - attr.value_range().start;
                        let mut target = String::new();
                        let mut flag = false;
                        for (i, character) in attr.value().chars().enumerate() {
                            if i == char_into_attr {
                                flag = true
                            }
                            match character.to_ascii_lowercase() {
                                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k'
                                | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u'
                                | 'v' | 'w' | 'x' | 'y' | 'z' | '$' => {
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
                        // let def_start = string.find(&target).unwrap();
                        // use target to get position of matching variable on our ecs world
                        let variable_storage = world.read_storage::<Variable>();
                        let file_storage = world.read_storage::<File>();
                        let span_storage = world.read_storage::<Span>();
                        for entity in world.entities().join() {
                            if let Some(file) = file_storage.get(entity) {
                                if file.path == file_path {
                                    if let Some(variable) = variable_storage.get(entity) {
                                        if variable.text == target {
                                            if let Some(span) = span_storage.get(entity) {
                                                info!("{:?}", span.start);
                                                let def_start_pos =
                                                    doc.text_pos_at(span.start.bytes);
                                                let diff = span.end.bytes - span.start.bytes;
                                                let def_range = Range {
                                                    start: Position::new(
                                                        def_start_pos.row as u64 - 1,
                                                        def_start_pos.col as u64 - 1,
                                                    ),
                                                    end: Position::new(
                                                        def_start_pos.row as u64 - 1,
                                                        (def_start_pos.col + diff as u32) as u64
                                                            - 1,
                                                    ),
                                                };
                                                locations.push(Location {
                                                    uri: file_uri.clone(),
                                                    range: def_range,
                                                });
                                                // break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                info!("line_doc failed");
            }
        }
    }
    locations
}

fn get_node_and_string(params: &GotoDefinitionParams) -> Option<(usize, String, PathBuf)> {
    if let Ok(path) = params
        .text_document_position_params
        .text_document
        .uri
        .to_file_path()
    {
        if let Ok(string) = fs::read_to_string(&path) {
            if let Some(line) = string
                .lines()
                .nth(params.text_document_position_params.position.line as usize)
            {
                if let Some(byte_position) = string.find(line) {
                    return Some((byte_position, string, path))
                }
            }
        }
    }
    None
}
