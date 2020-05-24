use crate::scriptproperties::Datatypes;

pub struct Event {
	name: String,
	description: String,
	object: Option<Vec<Datatypes>>,
	param: Option<Vec<Datatypes>>,
	param1: Option<Vec<Datatypes>>,
	param2: Option<Vec<Datatypes>>, 
	param3: Option<Vec<Datatypes>>
}