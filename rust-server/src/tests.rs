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
    #[test]
    fn test_recursive_parse() {
        let test = include_str!("reference/test_ref/md_namespace_basic.xml");
        let mut world = create_world();
        if let Ok(doc) = roxmltree::Document::parse(&test) {
            parse_doc(doc, &mut world, "path".to_string())
        }
    }
    #[test]
    fn test_expression_parse_basic() {
        let text1 = "if $Zone == player.zone then player.ship else $Zone".to_string();
        // let mut world = create_world();
        assert!(
            format!("{:?}", expression_parser::parse_expression(text1))
                == r#"[ExpValue { text: "$Zone", exp: Variable, start: 4, end: 8 }, ExpValue { text: "player", exp: Keyword, start: 13, end: 18 }, ExpValue { text: "zone", exp: ScriptProperty, start: 20, end: 23 }, ExpValue { text: "player", exp: Keyword, start: 30, end: 35 }, ExpValue { text: "ship", exp: ScriptProperty, start: 37, end: 40 }, ExpValue { text: "$Zone", exp: Variable, start: 47, end: 51 }]"#
        )
    }
    #[test]
    fn test_expression_parse_list() {
        let text3 = "[race.argon, race.paranid, race.teladi]".to_string();
        assert!(
            format!("{:?}", expression_parser::parse_expression(text3))
                == r#"[ExpValue { text: "race", exp: Keyword, start: 2, end: 5 }, ExpValue { text: "argon", exp: ScriptProperty, start: 7, end: 12 }, ExpValue { text: "race", exp: Keyword, start: 14, end: 17 }, ExpValue { text: "paranid", exp: ScriptProperty, start: 19, end: 26 }, ExpValue { text: "race", exp: Keyword, start: 28, end: 31 }, ExpValue { text: "teladi", exp: ScriptProperty, start: 33, end: 39 }]"#
        )
    }
    #[test]
    fn test_expression_parse_string() {
        let text4 = "'added ' + $Ship.knownname + ' to ' + (if $DockingBay.isstorage then 'storage dockingbay' else 'external dockingbay') + ' on dockarea.I love ' + $DockingBay.walkablemodule".to_string();
        assert!(
            format!("{:?}", expression_parser::parse_expression(text4))
                == r#"[ExpValue { text: "$Ship", exp: Variable, start: 12, end: 16 }, ExpValue { text: "knownname", exp: ScriptProperty, start: 18, end: 26 }, ExpValue { text: "$DockingBay", exp: Variable, start: 43, end: 53 }, ExpValue { text: "isstorage", exp: ScriptProperty, start: 55, end: 63 }, ExpValue { text: "$DockingBay", exp: Variable, start: 146, end: 156 }, ExpValue { text: "walkablemodule", exp: ScriptProperty, start: 158, end: 
             }]"#
        )
    }
    #[test]
    fn test_expression_parse_keys() {
        let text2 = "@$InternalShipsTable.{$DockArea}.{$ShipMacro.docksize} lt $InternalShipsQuota"
            .to_string();
        assert!(
            format!("{:?}", expression_parser::parse_expression(text2))
                == r#"[ExpValue { text: "$InternalShipsTable", exp: Variable, start: 2, end: 20 }, ExpValue { text: "$DockArea", exp: Key, start: 23, end: 31 }, ExpValue { text: "$ShipMacro", exp: Key, start: 35, end: 44 }, ExpValue { text: "docksize", exp: ScriptProperty, start: 46, end: 54 }, ExpValue { text: "$InternalShipsQuota", exp: Variable, start: 59, end: 77 }]"#
        )
    }
}
