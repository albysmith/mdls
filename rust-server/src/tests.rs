#[cfg(test)]
mod tests {
    use crate::completion_parser::*;

    use crate::definition_parser::*;

    use crate::type_checker::*;

    use crate::type_annotations::*;

    use crate::hover::*;

    use crate::expression_parser::*;

    use crate::scriptproperties::*;

    use crate::data_store::*;
    #[test]
    fn ron_methods() {
        let ron = parse_method_ron();
    }
    #[test]
    fn ron_events() {
        let ron = parse_event_ron();
    }
    #[test]
    fn scriptproperties() {
        let scripts = ScriptProperties::new(include_str!("reference/scriptproperties.xml"));
    }

}
