use log::info;
use lsp_types::{GotoDefinitionParams, Location, Position, Range};
use std::fs;

pub fn simple_definition(params: GotoDefinitionParams) -> Vec<Location> {
    info!("called simple_definition function");
    let mut locations = vec![];
    if let Some((byte_position, string)) = get_node_and_string(&params) {
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
                        let def_start = string.find(attr.value()).unwrap();
                        let def_start_pos = doc.text_pos_at(def_start);
                        let def_range = Range {
                            start: Position::new(
                                def_start_pos.row as u64 - 1,
                                def_start_pos.col as u64 - 1,
                            ),
                            end: Position::new(
                                def_start_pos.row as u64 - 1,
                                (def_start_pos.col + attr.value().len() as u32) as u64 - 1,
                            ),
                        };
                        // let def_location = Range {def_start..(def_start + attr.value().len());
                        locations.push(Location {
                            uri: params
                                .text_document_position_params
                                .text_document
                                .uri
                                .clone(),
                            range: def_range,
                        })
                    }
                }
            } else {
                info!("line_doc failed");
            }
        }
    }
    locations
}

fn get_node_and_string(params: &GotoDefinitionParams) -> Option<(usize, String)> {
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
