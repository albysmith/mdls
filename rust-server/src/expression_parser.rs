use crate::*;

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
            return Some(node_vars);
        }
    }

    None
}
#[derive(Debug, Clone)]
pub enum Expressions {
    Variable,
    ScriptProperty,
    Keyword,
    Dot,
    Key,
    Skip,
    StringQuote,
}

#[derive(Debug, Clone)]
pub struct ExpValue {
    text: String,
    exp: Expressions,
    start: usize,
    end: usize,
}

impl ExpValue {
    pub fn clear(&mut self) {
        self.text = String::new();
        self.exp = Expressions::Skip;
    }
    pub fn dot(&mut self) {
        self.text = String::new();
        self.exp = Expressions::Dot;
    }
    pub fn quote(&mut self) {
        self.text = String::new();
        self.exp = Expressions::StringQuote;
    }
    pub fn new(&mut self, exp: Expressions) {}
}

pub fn parse_expression(expression: String) -> Vec<ExpValue> {
    let mut expression_part = ExpValue {
        text: String::new(),
        exp: Expressions::Skip,
        start: 0,
        end: expression.len(),
    };
    let mut values = vec![];
    for (i, character) in expression.chars().enumerate() {
        match character {
            '$' => match expression_part.exp {
                Expressions::Skip | Expressions::Dot => {
                    expression_part.exp = Expressions::Variable;
                    expression_part.text = character.to_string();
                    expression_part.start = i + 1;
                }
                Expressions::StringQuote => {}
                Expressions::Key => expression_part.text.push_str(&character.to_string()),
                _ => {
                    expression_part.end = i;
                    values.push(expression_part.clone());
                    expression_part.exp = Expressions::Variable;
                    expression_part.text = character.to_string();
                    expression_part.start = i + 1;
                }
            },
            ' ' => match expression_part.exp {
                Expressions::StringQuote => {}
                Expressions::Variable | Expressions::ScriptProperty => {
                    expression_part.end = i;
                    values.push(expression_part.clone());
                    expression_part.clear();
                }
                Expressions::Keyword | Expressions::Key => expression_part.clear(),
                _ => expression_part.clear(),
            },
            '.' => match expression_part.exp {
                Expressions::StringQuote => {}
                Expressions::Variable
                | Expressions::ScriptProperty
                | Expressions::Keyword
                | Expressions::Key => {
                    expression_part.end = i;
                    values.push(expression_part.clone());
                    expression_part.dot();
                }
                _ => expression_part.dot(),
            },
            '{' => match expression_part.exp {
                Expressions::StringQuote => {}
                Expressions::Dot => {
                    expression_part.exp = Expressions::Key;
                    expression_part.text = String::new();
                    expression_part.start = i + 2;
                }
                _ => {}
            },
            '}' => match expression_part.exp {
                Expressions::StringQuote => {}
                Expressions::Key => {
                    expression_part.end = i;
                    values.push(expression_part.clone());
                    expression_part.clear()
                }
                _ => {}
            },
            '\'' => match expression_part.exp {
                Expressions::StringQuote => expression_part.clear(),
                _ => expression_part.quote(),
            },
            _ => {
                if character.is_alphanumeric() || character == '_' {
                    match expression_part.exp {
                        Expressions::StringQuote => {}
                        Expressions::Variable
                        | Expressions::ScriptProperty
                        | Expressions::Keyword
                        | Expressions::Key => expression_part.text.push_str(&character.to_string()),
                        Expressions::Skip => {
                            expression_part.exp = Expressions::Keyword;
                            expression_part.text = character.to_string();
                            expression_part.start = i + 1;
                        }
                        Expressions::Dot => {
                            expression_part.exp = Expressions::ScriptProperty;
                            expression_part.text = character.to_string();
                            expression_part.start = i + 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    match expression_part.exp {
        Expressions::Skip | Expressions::Dot => {}
        _ => {
            expression_part.end = expression.len();
            values.push(expression_part)
        }
    }
    values
}
