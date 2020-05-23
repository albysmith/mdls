use log::info;
use lsp_types::{Hover, HoverParams, Location, Position, Range};
use std::fs;

pub fn get_hover_value(params: HoverParams) -> Option<Hover> {
    info!("called hover function");
	// let mut hovers = vec![];
	// if let Some((_byte_position, string)) = get_node_and_string(&params) {
	// }

    Some(lsp_types::Hover {
        contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
            "Hello World!".to_string(),
        )),
        range: None,
    })
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
