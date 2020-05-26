#[cfg(test)]
mod tests {
    use crate::*;
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
    #[test]
    //test cant fail x.x
    // assert what namespace should be
    // better to rewrite namespace
    fn md_namespace_basic() {
        let test = include_str!("reference/test_ref/md_namespace_basic.xml");
        let namespace = parse_namespace((696, test.to_owned()));
        println!("{:#?}", namespace);
        assert!(namespace.is_some());
        assert!(namespace.unwrap().len() == 3)
    }
    fn definition_parsing() {}
}
