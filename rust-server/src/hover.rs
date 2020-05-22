use log::info;
use lsp_types::{Hover, HoverParams, Location, Position, Range};
use std::fs;

pub fn get_hover_value(params: HoverParams) -> Option<Hover> {
	


    Some(lsp_types::Hover {
        contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
            "Hello World!".to_string(),
        )),
        range: None,
    })
}
