use log::info;
use lsp_types::{CompletionItem, CompletionParams};
use std::fs;

//

pub fn simple_complete(params: CompletionParams) -> Vec<CompletionItem> {
    if let Some(result) = get_node_and_string(params) {
        if let Some(namespace) = parse_namespace(result) {
            return namespace;
        }
    }
    let namespace: Vec<CompletionItem> = vec![];
    namespace
}

pub fn parse_namespace(input: (usize, String)) -> Option<Vec<CompletionItem>> {
    let byte_position = input.0;
    let string = input.1;

    if let Ok(doc) = roxmltree::Document::parse(&string) {
        let mut namespace = vec![];
        for (_i, node) in doc.descendants().enumerate() {
            //filter against nodes that contain the whole fucking file in their range....
            match node.tag_name().name() {
                "mdscript" | "cues" | "cue" | "conditions" | "delay" | "actions" | "" => (),
                _ => {
                    if node.range().end >= byte_position {
                        info!(
                            "end node range:{} byte_positon: {} \n\n node name: {}",
                            node.range().end,
                            byte_position,
                            node.tag_name().name()
                        );
                        for ancestor in node.ancestors() {
                            info!("ancestor {}", ancestor.tag_name().name());
                            for f_child in ancestor.children() {
                                info!("f_child {}", f_child.tag_name().name());
                                match f_child.tag_name().name() {
                                    "actions" => {
                                        for actions_child in f_child.children() {
                                            info!(
                                                "actions_child {}",
                                                actions_child.tag_name().name()
                                            );
                                            if let Some(attr) = actions_child.attribute("name") {
                                                info!("COMPLETION ADDED {}", attr);
                                                add_completion_item(&mut namespace, attr);
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        return Some(namespace);
    }
    None
}

// pub fn simple_complete(params: CompletionParams) -> Vec<CompletionItem> {
//     let mut namespace = vec![];
//     if let Some((byte_position, string)) = get_node_and_string(params) {
//         if let Ok(doc) = roxmltree::Document::parse(&string) {
//             for (_i, node) in doc.descendants().enumerate() {
//                 //filter against nodes that contain the whole fucking file in their range....
//                 match node.tag_name().name() {
//                     "mdscript" | "cues" | "cue" | "conditions" | "delay" | "actions" | "" => (),
//                     _ => {
//                         if node.range().end >= byte_position {
//                             info!(
//                                 "end node range:{} byte_positon: {} \n\n node name: {}",
//                                 node.range().end,
//                                 byte_position,
//                                 node.tag_name().name()
//                             );
//                             for ancestor in node.ancestors() {
//                                 info!("ancestor {}", ancestor.tag_name().name());
//                                 for f_child in ancestor.first_children() {
//                                     info!("f_child {}", f_child.tag_name().name());
//                                     match f_child.tag_name().name() {
//                                         "actions" => {
//                                             for actions_child in f_child.children() {
//                                                 info!(
//                                                     "actions_child {}",
//                                                     actions_child.tag_name().name()
//                                                 );
//                                                 if let Some(attr) = actions_child.attribute("name")
//                                                 {
//                                                     info!("COMPLETION ADDED {}", attr);
//                                                     add_completion_item(&mut namespace, attr);
//                                                 }
//                                             }
//                                         }
//                                         _ => (),
//                                     }
//                                 }
//                             }
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     namespace
// }

fn add_completion_item(namespace: &mut Vec<CompletionItem>, attr: &str) {
    let item = CompletionItem {
        label: attr.to_owned(),
        ..CompletionItem::default()
    };
    if !namespace.contains(&item) {
        namespace.push(item);
    }
}
fn get_node_and_string(params: CompletionParams) -> Option<(usize, String)> {
    if let Ok(path) = params
        .text_document_position
        .text_document
        .uri
        .to_file_path()
    {
        if let Ok(string) = fs::read_to_string(path) {
            if let Some(line) = string
                .lines()
                .nth(params.text_document_position.position.line as usize)
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
