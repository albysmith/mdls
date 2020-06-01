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
    pub text: String,
    pub exp: Expressions,
    pub start: usize,
    pub end: usize,
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
#[derive(Debug, Clone)]

pub struct TypeData {
    pub exp_type: expression_parser::Expressions,
    pub property: Option<Property>,
    pub variable: Option<Datatypes>,
    pub keyword: Option<Datatypes>,
}

pub fn infer_types(
    var: &components::Variable,
    node: &components::Node,
    exp_value: &expression_parser::ExpValue,
    prior_type: &Vec<Datatypes>,
    scriptps: Vec<Property>,
) -> Option<(Vec<TypeData>, Vec<Datatypes>)> {
    match exp_value.exp {
        Expressions::Variable | Expressions::Key => {
            let mut types = vec![];
            let mut current_types = vec![];
            if let Some(data_types) = get_method_types(var, node) {
                for d in data_types.iter() {
                    types.push(TypeData {
                        exp_type: exp_value.exp.to_owned(),
                        property: None,
                        variable: Some(d.to_owned()),
                        keyword: None,
                    })
                }
                current_types = data_types;
            }
            if types.len() > 0 {
                return Some((types, current_types));
            }
            None
        }
        Expressions::ScriptProperty => {
            let mut types = vec![];
            let mut current_types = vec![];
            for prior in prior_type.into_iter() {
                for entry in &scriptps {
                    if entry.datatype == prior.to_owned() {
                        if let Some(prop_type) = &entry.prop_type {
                            current_types.push(match_datatype(Some(prop_type)));
                        }
                        types.push(TypeData {
                            exp_type: exp_value.exp.to_owned(),
                            property: Some(entry.to_owned()),
                            variable: None,
                            keyword: None,
                        })
                    }
                }
            }
            if types.len() > 0 {
                return Some((types, current_types));
            } else {
                
            }
            None
        }
        Expressions::Keyword => None,
        Expressions::Dot | Expressions::Skip | Expressions::StringQuote => None,
    }
}

// limit system to only current file; hook up to notification for file update?
// if nothing matches the prior type, then show all?
// figure out scriptproperty inheritance........
// add variable handling with namespace checking, etc. like in Hover
    // currently only handles variable creation point, which is useless for scriptproperties

fn get_method_types(var: &components::Variable, node: &components::Node) -> Option<Vec<Datatypes>> {
    if let Some(method) = &node.method {
        for output in method.output.iter() {
            if output.attr == var.name {
                if let Some(types) = &output.datatype {
                    let mut type_vec = vec![];
                    for value in types {
                        type_vec.push(value.to_owned());
                    }
                    if let Some(types) = &output.contains {
                        for value in types {
                            if !type_vec.contains(value) {
                                type_vec.push(value.to_owned());
                            }
                        }
                        return Some(type_vec);
                    }
                }
            }
        }
    }
    None
}
