use crate::scriptproperties::Datatypes;

pub struct Event {
	pub id: String,
	pub description: String,
	pub object: Option<Vec<Datatypes>>,
	pub param: Option<Vec<Datatypes>>,
	pub param1: Option<Vec<Datatypes>>,
	pub param2: Option<Vec<Datatypes>>, 
	pub param3: Option<Vec<Datatypes>>
}

pub struct EventList {
	pub events: Vec<Event>
}


pub struct Method {
	pub id: String,
	pub description: String,
	pub output: Vec<Output>
}

pub struct MethodList {
	methods: Vec<Method>
}

pub struct Output {
	pub attr: String,
	pub datatype: Option<Vec<Datatypes>>,
	pub contains: Option<Vec<Datatypes>>,
}

pub fn parse_ron_files() {
	let methods = include_str!("reference/methods.ron")
}