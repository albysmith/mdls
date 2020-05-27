#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn ron_methods() {
        let _ron = parse_method_ron();
    }
    #[test]
    fn ron_events() {
        let _ron = parse_event_ron();
    }
    #[test]
    fn scriptproperties() {
        let _scripts = ScriptProperties::new(include_str!("reference/scriptproperties.xml"));
    }
    #[test]
    //test cant fail x.x
    // assert what namespace should be
    fn md_namespace_basic() {
        let test = include_str!("reference/test_ref/md_namespace_basic.xml");
        let namespace = parse_namespace((696, test.to_owned()));
        println!("{:#?}", namespace);
        assert!(namespace.is_some());
        assert!(namespace.unwrap().len() == 3)
    }
}
