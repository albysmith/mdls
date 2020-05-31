use crate::scriptproperties::Datatypes;
use serde::Deserialize;
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event {
    pub id: String,
    pub description: String,
    pub object: Option<Vec<Datatypes>>,
    pub param: Option<Vec<Datatypes>>,
    pub param1: Option<Vec<Datatypes>>,
    pub param2: Option<Vec<Datatypes>>,
    pub param3: Option<Vec<Datatypes>>,
}
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventList {
    pub events: Vec<Event>,
}
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Method {
    pub id: String,
    pub description: String,
    pub output: Vec<Output>,
}

// impl Method {
//     pub fn match_node_type() -> Vec<Datatypes>{

//     }
// }

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MethodList {
    pub methods: Vec<Method>,
}
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Output {
    pub attr: String,
    pub datatype: Option<Vec<Datatypes>>,
    pub contains: Option<Vec<Datatypes>>,
}

pub fn parse_method_ron() -> MethodList {
    let string = include_str!("reference/methods.ron");
    let methods: MethodList = match ron::from_str(string) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    methods
}
pub fn parse_event_ron() -> EventList {
    let string = include_str!("reference/events.ron");
    let events: EventList = match ron::from_str(string) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    events
}
