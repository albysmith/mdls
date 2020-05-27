#[derive(Debug, Clone)]
struct NodeVars<'b> {
    node: &'b str,
    vars: Vec<&'b str>,
}

fn _find_vars<'a>(doc: &'a roxmltree::Document) -> Option<NodeVars<'a>> {
    let expressions = doc
        .descendants()
        .map(|n| (n.attribute("exact"), n.tag_name().name()))
        .collect::<Vec<_>>();
    for node in expressions.iter() {
        if let Some(exp) = node.0 {
            let variables = exp
                .split_terminator(|c| {
                    c == '='
                        || c == '.'
                        || c == ','
                        || c == '['
                        || c == ']'
                        || c == '{'
                        || c == '}'
                        || c == ' '
                        || c == '@'
                        || c == '('
                        || c == ')'
                })
                .collect::<Vec<_>>();
            let node_vars = NodeVars {
                node: node.1,
                vars: variables,
            };
            // println!("{:#?}", node_vars);
            return Some(node_vars);
        }
    }

    None
}
