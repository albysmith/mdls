use serde::Deserialize;
use std::fs;
use std::iter;

#[derive(Debug, Clone)]
pub struct ScriptProperties {
    pub data: Vec<ApplicableProperties>,
}

impl ScriptProperties {
    pub fn new(string: &str) -> Self {
        let doc = roxmltree::Document::parse(&string).expect("malformed xml i guess");
        let mut sp = ScriptProperties { data: vec![] };
        let mut ap = ApplicableProperties {
            datatype: Datatypes::Activity,
            properties: vec![],
        };
        for node in doc.descendants() {
            match node.tag_name().name() {
                "datatype" | "keyword" => {
                    // last keyword will never be pushed
                    sp.data.push(ap.clone());
                    ap.properties.clear();
                    ap.datatype = match_datatype(node.attribute("name"));
                }
                "property" => ap.properties.push(Property {
                    datatype: ap.datatype.clone(),
                    prop_name: if let Some(stir) = node.attribute("name") {
                        Some(stir.to_string())
                    } else {
                        None
                    },
                    prop_result: if let Some(stir) = node.attribute("result") {
                        Some(stir.to_string())
                    } else {
                        None
                    },
                    prop_type: if let Some(stir) = node.attribute("type") {
                        Some(stir.to_string())
                    } else {
                        None
                    }, // keywords have description instead of result (among others)
                }),
                _ => (),
            }
        }
        sp
    }
    pub fn search(&self, string: &str) -> Vec<Property> {
        let properties = self.data.iter().filter_map(|x| x.search(string)).collect();
        // for datatype in self.data.iter() {
        //     if let Some(prop) = datatype.search(string) {

        //     }
        // }
        properties
    }
}

fn match_datatype(string: Option<&str>) -> Datatypes {
    match string.expect("datatype or keyword didn't have a name field") {
        "activity" => Datatypes::Activity,
        "adsign" => Datatypes::Adsign,
        "alertlevel" => Datatypes::Alertlevel,
        "angle" => Datatypes::Angle,
        "assignment" => Datatypes::Assignment,
        "attention" => Datatypes::Attention,
        "blacklistgroup" => Datatypes::Blacklistgroup,
        "blacklisttype" => Datatypes::Blacklisttype,
        "boardingbehaviour" => Datatypes::Boardingbehaviour,
        "boardingphase" => Datatypes::Boardingphase,
        "boolean" => Datatypes::Boolean,
        "build" => Datatypes::Build,
        "buildmodule" => Datatypes::Buildmodule,
        "buildprocessor" => Datatypes::Buildprocessor,
        "buildstorage" => Datatypes::Buildstorage,
        "cargolist" => Datatypes::Cargolist,
        "class" => Datatypes::Class,
        "cluster" => Datatypes::Cluster,
        "collectable" => Datatypes::Collectable,
        "command" => Datatypes::Command,
        "commandaction" => Datatypes::Commandaction,
        "component" => Datatypes::Component,
        "componentslot" => Datatypes::Componentslot,
        "componentstate" => Datatypes::Componentstate,
        "constructionplanentrydata" => Datatypes::Constructionplanentrydata,
        "constructionplanentryid" => Datatypes::Constructionplanentryid,
        "constructionsequence" => Datatypes::Constructionsequence,
        "container" => Datatypes::Container,
        "controllable" => Datatypes::Controllable,
        "controlpaneltype" => Datatypes::Controlpaneltype,
        "controlposition" => Datatypes::Controlposition,
        "controlpost" => Datatypes::Controlpost,
        "cue" => Datatypes::Cue,
        "cuestate" => Datatypes::Cuestate,
        "datatype" => Datatypes::Datatype,
        "dbdata" => Datatypes::Dbdata,
        "defensible" => Datatypes::Defensible,
        "deployablecategory" => Datatypes::Deployablecategory,
        "destructible" => Datatypes::Destructible,
        "dockarea" => Datatypes::Dockarea,
        "dockingbay" => Datatypes::Dockingbay,
        "dockstate" => Datatypes::Dockstate,
        "dronemode" => Datatypes::Dronemode,
        "drop" => Datatypes::Drop,
        "engine" => Datatypes::Engine,
        "entity" => Datatypes::Entity,
        "entityrole" => Datatypes::Entityrole,
        "entitytype" => Datatypes::Entitytype,
        "enum" => Datatypes::Enum,
        "explosive" => Datatypes::Explosive,
        "faction" => Datatypes::Faction,
        "flightbehaviour" => Datatypes::Flightbehaviour,
        "flightcontrolmodel" => Datatypes::Flightcontrolmodel,
        "float" => Datatypes::Float,
        "formationshape" => Datatypes::Formationshape,
        "galaxy" => Datatypes::Galaxy,
        "gate" => Datatypes::Gate,
        "group" => Datatypes::Group,
        "highway" => Datatypes::Highway,
        "highwayentrygate" => Datatypes::Highwayentrygate,
        "highwayexitgate" => Datatypes::Highwayexitgate,
        "hitpoints" => Datatypes::Hitpoints,
        "integer" => Datatypes::Integer,
        "killmethod" => Datatypes::Killmethod,
        "kwenum" => Datatypes::Kwenum,
        "largefloat" => Datatypes::Largefloat,
        "largeint" => Datatypes::Largeint,
        "length" => Datatypes::Length,
        "level" => Datatypes::Level,
        "licence" => Datatypes::Licence,
        "list" => Datatypes::List,
        "loadout" => Datatypes::Loadout,
        "lock" => Datatypes::Lock,
        "lookuplist" => Datatypes::Lookuplist,
        "macro" => Datatypes::Macro,
        "missiongroup" => Datatypes::Missiongroup,
        "missiontype" => Datatypes::Missiontype,
        "module" => Datatypes::Module,
        "money" => Datatypes::Money,
        "moodlevel" => Datatypes::Moodlevel,
        "navcontext" => Datatypes::Navcontext,
        "nonplayer" => Datatypes::Nonplayer,
        "notification" => Datatypes::Notification,
        "npc" => Datatypes::NPC,
        "npctemplate" => Datatypes::Npctemplate,
        "npctemplateentry" => Datatypes::Npctemplateentry,
        "numeric" => Datatypes::Numeric,
        "object" => Datatypes::Object,
        "objective" => Datatypes::Objective,
        "operation" => Datatypes::Operation,
        "order" => Datatypes::Order,
        "orderstate" => Datatypes::Orderstate,
        "pier" => Datatypes::Pier,
        "position" => Datatypes::Position,
        "purpose" => Datatypes::Purpose,
        "quadrant" => Datatypes::Quadrant,
        "race" => Datatypes::Race,
        "region" => Datatypes::Region,
        "relationchangereason" => Datatypes::Relationchangereason,
        "room" => Datatypes::Room,
        "roompopulationtype" => Datatypes::Roompopulationtype,
        "roomtype" => Datatypes::Roomtype,
        "rotation" => Datatypes::Rotation,
        "sector" => Datatypes::Sector,
        "shieldgenerator" => Datatypes::Shieldgenerator,
        "ship" => Datatypes::Ship,
        "shiptype" => Datatypes::Shiptype,
        "signalleak" => Datatypes::Signalleak,
        "signalleaktype" => Datatypes::Signalleaktype,
        "skilltype" => Datatypes::Skilltype,
        "space" => Datatypes::Space,
        "spacesuit" => Datatypes::Spacesuit,
        "station" => Datatypes::Station,
        "string" => Datatypes::String,
        "table" => Datatypes::Table,
        "tag" => Datatypes::Tag,
        "time" => Datatypes::Time,
        "trade" => Datatypes::Trade,
        "turret" => Datatypes::Turret,
        "unitcategory" => Datatypes::Unitcategory,
        "unlock" => Datatypes::Unlock,
        "vector" => Datatypes::Vector,
        "ventureplatform" => Datatypes::Ventureplatform,
        "walkablemodule" => Datatypes::Walkablemodule,
        "ware" => Datatypes::Ware,
        "wareamountlist" => Datatypes::Wareamountlist,
        "warelist" => Datatypes::Warelist,
        "waretransport" => Datatypes::Waretransport,
        "weapon" => Datatypes::Weapon,
        "weaponmode" => Datatypes::Weaponmode,
        "zone" => Datatypes::Zone,
        "player" => Datatypes::Player,
        "global" => Datatypes::Global,
        "true" => Datatypes::True,
        "false" => Datatypes::False,
        "pi" => Datatypes::Pi,
        "null" => Datatypes::Null,
        "readtext" => Datatypes::Readtext,
        "stat" => Datatypes::Stat,
        "userdata" => Datatypes::Userdata,
        "param" => Datatypes::Param,
        "loop" => Datatypes::Loop,
        "warebasket" => Datatypes::Warebasket,
        "waregroup" => Datatypes::Waregroup,
        "lookup" => Datatypes::Lookup,
        "this" => Datatypes::This,
        "event" => Datatypes::Event,
        "quota" => Datatypes::Quota,
        "md" => Datatypes::Md,
        "parent" => Datatypes::Parent,
        "static" => Datatypes::Static,
        "staticbase" => Datatypes::Staticbase,
        "namespace" => Datatypes::Namespace,
        _ => Datatypes::Unknown,
    }
}

#[derive(Debug, Clone)]
pub struct ApplicableProperties {
    pub datatype: Datatypes,
    pub properties: Vec<Property>,
}
impl ApplicableProperties {
    fn search(&self, string: &str) -> Option<Property> {
        for prop in self.properties.iter() {
            if let Some(name) = prop.clone().prop_name {
                if string.to_string() == name {
                    return Some(prop.clone());
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub datatype: Datatypes,
    pub prop_name: Option<String>,
    pub prop_result: Option<String>,
    pub prop_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Datatypes {
    Activity,
    Adsign,
    Alertlevel,
    Angle,
    Assignment,
    Attention,
    Blacklistgroup,
    Blacklisttype,
    Boardingbehaviour,
    Boardingphase,
    Boolean,
    Build,
    Buildmodule,
    Buildprocessor,
    Buildstorage,
    Cargolist,
    Class,
    Cluster,
    Collectable,
    Command,
    Commandaction,
    Component,
    Componentslot,
    Componentstate,
    Constructionplanentrydata,
    Constructionplanentryid,
    Constructionsequence,
    Container,
    Controllable,
    Controlpaneltype,
    Controlposition,
    Controlpost,
    Cue,
    Cuestate,
    Datatype,
    Dbdata,
    Defensible,
    Deployablecategory,
    Destructible,
    Dockarea,
    Dockingbay,
    Dockstate,
    Dronemode,
    Drop,
    Engine,
    Entity,
    Entityrole,
    Entitytype,
    Enum,
    Explosive,
    Faction,
    Flightbehaviour,
    Flightcontrolmodel,
    Float,
    Formationshape,
    Galaxy,
    Gate,
    Group,
    Highway,
    Highwayentrygate,
    Highwayexitgate,
    Hitpoints,
    Integer,
    Killmethod,
    Kwenum,
    Largefloat,
    Largeint,
    Length,
    Level,
    Licence,
    List,
    Loadout,
    Lock,
    Lookuplist,
    Macro,
    Missiongroup,
    Missiontype,
    Module,
    Money,
    Moodlevel,
    Navcontext,
    NPC,
    Nonplayer,
    Notification,
    Npctemplate,
    Npctemplateentry,
    Numeric,
    Object,
    Objective,
    Operation,
    Order,
    Orderstate,
    Pier,
    Position,
    Purpose,
    Quadrant,
    Race,
    Region,
    Relationchangereason,
    Room,
    Roompopulationtype,
    Roomtype,
    Rotation,
    Sector,
    Shieldgenerator,
    Ship,
    Shiptype,
    Signalleak,
    Signalleaktype,
    Skilltype,
    Space,
    Spacesuit,
    Station,
    String,
    Table,
    Tag,
    Time,
    Trade,
    Turret,
    Unitcategory,
    Unlock,
    Vector,
    Ventureplatform,
    Walkablemodule,
    Ware,
    Wareamountlist,
    Warelist,
    Waretransport,
    Weapon,
    Weaponmode,
    Zone,
    Player,
    Global,
    True,
    False,
    Pi,
    Null,
    Readtext,
    Stat,
    Userdata,
    Param,
    Loop,
    Warebasket,
    Waregroup,
    Lookup,
    This,
    Event,
    Quota,
    Md,
    Parent,
    Static,
    Staticbase,
    Namespace,
    Unknown,
}

// impl iter::Iterator for Datatypes {
//     type Item = Self;
//     fn next(&mut self) -> Option<Self> {
//         use Datatypes::*;
//         match self {
//             Activity => Some(Adsign),
//             Adsign => Some(Alertlevel),
//             Alertlevel => Some(Angle),
//             Angle => Some(Assignment),
//             Assignment => Some(Attention),
//             Attention => Some(Blacklistgroup),
//             Blacklistgroup => Some(Blacklisttype),
//             Blacklisttype => Some(Boardingbehaviour),
//             Boardingbehaviour => Some(Boardingphase),
//             Boardingphase => Some(Boolean),
//             Boolean => Some(Build),
//             Build => Some(Buildmodule),
//             Buildmodule => Some(Buildprocessor),
//             Buildprocessor => Some(Buildstorage),
//             Buildstorage => Some(Cargolist),
//             Cargolist => Some(Class),
//             Class => Some(Cluster),
//             Cluster => Some(Collectable),
//             Collectable => Some(Command),
//             Command => Some(Commandaction),
//             Commandaction => Some(Component),
//             Component => Some(Componentslot),
//             Componentslot => Some(Componentstate),
//             Componentstate => Some(Constructionplanentrydata),
//             Constructionplanentrydata => Some(Constructionplanentryid),
//             Constructionplanentryid => Some(Constructionsequence),
//             Constructionsequence => Some(Container),
//             Container => Some(Controllable),
//             Controllable => Some(Controlpaneltype),
//             Controlpaneltype => Some(Controlposition),
//             Controlposition => Some(Controlpost),
//             Controlpost => Some(Cue),
//             Cue => Some(Cuestate),
//             Cuestate => Some(Datatype),
//             Datatype => Some(Dbdata),
//             Dbdata => Some(Defensible),
//             Defensible => Some(Deployablecategory),
//             Deployablecategory => Some(Destructible),
//             Destructible => Some(Dockarea),
//             Dockarea => Some(Dockingbay),
//             Dockingbay => Some(Dockstate),
//             Dockstate => Some(Dronemode),
//             Dronemode => Some(Drop),
//             Drop => Some(Engine),
//             Engine => Some(Entity),
//             Entity => Some(Entityrole),
//             Entityrole => Some(Entitytype),
//             Entitytype => Some(Enum),
//             Enum => Some(Explosive),
//             Explosive => Some(Faction),
//             Faction => Some(Flightbehaviour),
//             Flightbehaviour => Some(Flightcontrolmodel),
//             Flightcontrolmodel => Some(Float),
//             Float => Some(Formationshape),
//             Formationshape => Some(Galaxy),
//             Galaxy => Some(Gate),
//             Gate => Some(Group),
//             Group => Some(Highway),
//             Highway => Some(Highwayentrygate),
//             Highwayentrygate => Some(Highwayexitgate),
//             Highwayexitgate => Some(Hitpoints),
//             Hitpoints => Some(Integer),
//             Integer => Some(Killmethod),
//             Killmethod => Some(Kwenum),
//             Kwenum => Some(Largefloat),
//             Largefloat => Some(Largeint),
//             Largeint => Some(Length),
//             Length => Some(Level),
//             Level => Some(Licence),
//             Licence => Some(List),
//             List => Some(Loadout),
//             Loadout => Some(Lock),
//             Lock => Some(Lookuplist),
//             Lookuplist => Some(Macro),
//             Macro => Some(Missiongroup),
//             Missiongroup => Some(Missiontype),
//             Missiontype => Some(Module),
//             Module => Some(Money),
//             Money => Some(Moodlevel),
//             Moodlevel => Some(Navcontext),
//             Navcontext => Some(Nonplayer),
//             Nonplayer => Some(Notification),
//             Notification => Some(Npctemplate),
//             Npctemplate => Some(Npctemplateentry),
//             Npctemplateentry => Some(Numeric),
//             Numeric => Some(Object),
//             Object => Some(Objective),
//             Objective => Some(Operation),
//             Operation => Some(Order),
//             Order => Some(Orderstate),
//             Orderstate => Some(Pier),
//             Pier => Some(Position),
//             Position => Some(Purpose),
//             Purpose => Some(Quadrant),
//             Quadrant => Some(Race),
//             Race => Some(Region),
//             Region => Some(Relationchangereason),
//             Relationchangereason => Some(Room),
//             Room => Some(Roompopulationtype),
//             Roompopulationtype => Some(Roomtype),
//             Roomtype => Some(Rotation),
//             Rotation => Some(Sector),
//             Sector => Some(Shieldgenerator),
//             Shieldgenerator => Some(Ship),
//             Ship => Some(Shiptype),
//             Shiptype => Some(Signalleaktype),
//             Signalleaktype => Some(Skilltype),
//             Skilltype => Some(Space),
//             Space => Some(Spacesuit),
//             Spacesuit => Some(Station),
//             Station => Some(String),
//             String => Some(Table),
//             Table => Some(Tag),
//             Tag => Some(Time),
//             Time => Some(Trade),
//             Trade => Some(Turret),
//             Turret => Some(Unitcategory),
//             Unitcategory => Some(Unlock),
//             Unlock => Some(Vector),
//             Vector => Some(Ventureplatform),
//             Ventureplatform => Some(Walkablemodule),
//             Walkablemodule => Some(Ware),
//             Ware => Some(Wareamountlist),
//             Wareamountlist => Some(Warelist),
//             Warelist => Some(Waretransport),
//             Waretransport => Some(Weapon),
//             Weapon => Some(Weaponmode),
//             Weaponmode => Some(Zone),
//             Zone => None,
//         }
//     }
// }

/*
    1)


*/

// struct Component {
//     applicable_properties: Vec<Property>,
// }
// struct Destructible {
//     applicable_properties: Vec<Property>,
// }
// struct Object {
//     applicable_properties: Vec<Property>,
// }
// struct Mine {
//     applicable_properties: Vec<Property>,
// }
// struct Controllable {
//     applicable_properties: Vec<Property>,
// }
// struct Defensible {
//     applicable_properties: Vec<Property>,
// }
// struct Container {
//     applicable_properties: Vec<Property>,
// }
// struct Ship {
//     applicable_properties: Vec<Property>,
// }
// struct Spacesuit {
//     applicable_properties: Vec<Property>,
// }
// struct Station {
//     applicable_properties: Vec<Property>,
// }
// struct Explosive {
//     applicable_properties: Vec<Property>,
// }
// struct Bullet {
//     applicable_properties: Vec<Property>,
// }
// struct Missile {
//     applicable_properties: Vec<Property>,
// }
// struct Gate {
//     applicable_properties: Vec<Property>,
// }
// struct Highwayentrygate {
//     applicable_properties: Vec<Property>,
// }
// struct Highwayexitgate {
//     applicable_properties: Vec<Property>,
// }
// struct Drop {
//     applicable_properties: Vec<Property>,
// }
// struct Satellite {
//     applicable_properties: Vec<Property>,
// }
// struct Navbeacon {
//     applicable_properties: Vec<Property>,
// }
// struct Resourceprobe {
//     applicable_properties: Vec<Property>,
// }
// struct Lockbox {
//     applicable_properties: Vec<Property>,
// }
// struct Lock {
//     applicable_properties: Vec<Property>,
// }
// struct Collectable {
//     applicable_properties: Vec<Property>,
// }
// struct Collectableammo {
//     applicable_properties: Vec<Property>,
// }
// struct Collectablewares {
//     applicable_properties: Vec<Property>,
// }
// struct Buildstorage {
//     applicable_properties: Vec<Property>,
// }
// struct Adsign {
//     applicable_properties: Vec<Property>,
// }
// struct Shieldgenerator {
//     applicable_properties: Vec<Property>,
// }
// struct Navcontext {
//     applicable_properties: Vec<Property>,
// }
// struct Module {
//     applicable_properties: Vec<Property>,
// }
// struct Buildmodule {
//     applicable_properties: Vec<Property>,
// }
// struct Buildprocessor {
//     applicable_properties: Vec<Property>,
// }
// struct Connectionmodule {
//     applicable_properties: Vec<Property>,
// }
// struct Defencemodule {
//     applicable_properties: Vec<Property>,
// }
// struct Ventureplatform {
//     applicable_properties: Vec<Property>,
// }
// struct Habitationmodule {
//     applicable_properties: Vec<Property>,
// }
// struct Pier {
//     applicable_properties: Vec<Property>,
// }
// struct Production {
//     applicable_properties: Vec<Property>,
// }
// struct Storagemodule {
//     applicable_properties: Vec<Property>,
// }
// struct Engine {
//     applicable_properties: Vec<Property>,
// }
// struct Scanner {
//     applicable_properties: Vec<Property>,
// }
// struct Crate {
//     applicable_properties: Vec<Property>,
// }
// struct Controlpanel {
//     applicable_properties: Vec<Property>,
// }
// struct Signalleak {
//     applicable_properties: Vec<Property>,
// }
// struct Room {
//     applicable_properties: Vec<Property>,
// }
// struct Dockingbay {
//     applicable_properties: Vec<Property>,
// }
// struct Walkablemodule {
//     applicable_properties: Vec<Property>,
// }
// struct Dockarea {
//     applicable_properties: Vec<Property>,
// }
// struct Space {
//     applicable_properties: Vec<Property>,
// }
// struct Galaxy {
//     applicable_properties: Vec<Property>,
// }
// struct Cluster {
//     applicable_properties: Vec<Property>,
// }
// struct Sector {
//     applicable_properties: Vec<Property>,
// }
// struct Zone {
//     applicable_properties: Vec<Property>,
// }
// struct Highway {
//     applicable_properties: Vec<Property>,
// }
// struct Region {
//     applicable_properties: Vec<Property>,
// }
// struct Entity {
//     applicable_properties: Vec<Property>,
// }
// struct Nonplayer {
//     applicable_properties: Vec<Property>,
// }
// struct Npc {
//     applicable_properties: Vec<Property>,
// }
// struct Weapon {
//     applicable_properties: Vec<Property>,
// }
// struct Turret {
//     applicable_properties: Vec<Property>,
// }
// struct Componentslot {
//     applicable_properties: Vec<Property>,
// }
// struct Macroslot {
//     applicable_properties: Vec<Property>,
// }
// struct Trade {
//     applicable_properties: Vec<Property>,
// }
// struct Build {
//     applicable_properties: Vec<Property>,
// }
// struct Operation {
//     applicable_properties: Vec<Property>,
// }
// struct Licence {
//     applicable_properties: Vec<Property>,
// }
// struct Group {
//     applicable_properties: Vec<Property>,
// }
// struct Order {
//     applicable_properties: Vec<Property>,
// }
// struct Constructionsequence {
//     applicable_properties: Vec<Property>,
// }
// struct Constructionplanentryid {
//     applicable_properties: Vec<Property>,
// }
// struct Constructionplanentrydata {
//     applicable_properties: Vec<Property>,
// }
// struct Loadout {
//     applicable_properties: Vec<Property>,
// }
// struct List {
//     applicable_properties: Vec<Property>,
// }
// struct Table {
//     applicable_properties: Vec<Property>,
// }
// struct String {
//     applicable_properties: Vec<Property>,
// }
// struct Position {
//     applicable_properties: Vec<Property>,
// }
// struct Vector {
//     applicable_properties: Vec<Property>,
// }
// struct Rotation {
//     applicable_properties: Vec<Property>,
// }
// struct Faction {
//     applicable_properties: Vec<Property>,
// }
// struct Race {
//     applicable_properties: Vec<Property>,
// }
// struct Ware {
//     applicable_properties: Vec<Property>,
// }
// struct Lookuplist {
//     applicable_properties: Vec<Property>,
// }
// struct Warelist {
//     applicable_properties: Vec<Property>,
// }
// struct Wareamountlist {
//     applicable_properties: Vec<Property>,
// }
// struct Cargolist {
//     applicable_properties: Vec<Property>,
// }
// struct Npctemplate {
//     applicable_properties: Vec<Property>,
// }
// struct Npctemplateentry {
//     applicable_properties: Vec<Property>,
// }
// struct Macro {
//     applicable_properties: Vec<Property>,
// }
// struct Unlock {
//     applicable_properties: Vec<Property>,
// }
// struct Missiongroup {
//     applicable_properties: Vec<Property>,
// }
// struct Notification {
//     applicable_properties: Vec<Property>,
// }
// struct Tag {
//     applicable_properties: Vec<Property>,
// }
// struct Cue {
//     applicable_properties: Vec<Property>,
// }
// struct Enum {
//     applicable_properties: Vec<Property>,
// }
// struct Kwenum {
//     applicable_properties: Vec<Property>,
// }
// struct Dbdata {
//     applicable_properties: Vec<Property>,
// }
// struct Class {
//     applicable_properties: Vec<Property>,
// }
// struct Componentstate {
//     applicable_properties: Vec<Property>,
// }
// struct Killmethod {
//     applicable_properties: Vec<Property>,
// }
// struct Attention {
//     applicable_properties: Vec<Property>,
// }
// struct Flightbehaviour {
//     applicable_properties: Vec<Property>,
// }
// struct Flightcontrolmodel {
//     applicable_properties: Vec<Property>,
// }
// struct Formationshape {
//     applicable_properties: Vec<Property>,
// }
// struct Activity {
//     applicable_properties: Vec<Property>,
// }
// struct Objective {
//     applicable_properties: Vec<Property>,
// }
// struct Missiontype {
//     applicable_properties: Vec<Property>,
// }
// struct Level {
//     applicable_properties: Vec<Property>,
// }
// struct Alertlevel {
//     applicable_properties: Vec<Property>,
// }
// struct Command {
//     applicable_properties: Vec<Property>,
// }
// struct Commandaction {
//     applicable_properties: Vec<Property>,
// }
// struct Cuestate {
//     applicable_properties: Vec<Property>,
// }
// struct Dronemode {
//     applicable_properties: Vec<Property>,
// }
// struct Roompopulationtype {
//     applicable_properties: Vec<Property>,
// }
// struct Purpose {
//     applicable_properties: Vec<Property>,
// }
// struct Moodlevel {
//     applicable_properties: Vec<Property>,
// }
// struct Orderstate {
//     applicable_properties: Vec<Property>,
// }
// struct Weaponmode {
//     applicable_properties: Vec<Property>,
// }
// struct Dockstate {
//     applicable_properties: Vec<Property>,
// }
// struct Quadrant {
//     applicable_properties: Vec<Property>,
// }
// struct Datatype {
//     applicable_properties: Vec<Property>,
// }
// struct Entitytype {
//     applicable_properties: Vec<Property>,
// }
// struct Entityrole {
//     applicable_properties: Vec<Property>,
// }
// struct Skilltype {
//     applicable_properties: Vec<Property>,
// }
// struct Controlpost {
//     applicable_properties: Vec<Property>,
// }
// struct Assignment {
//     applicable_properties: Vec<Property>,
// }
// struct Controlposition {
//     applicable_properties: Vec<Property>,
// }
// struct Unitcategory {
//     applicable_properties: Vec<Property>,
// }
// struct Deployablecategory {
//     applicable_properties: Vec<Property>,
// }
// struct Controlpaneltype {
//     applicable_properties: Vec<Property>,
// }
// struct Signalleaktype {
//     applicable_properties: Vec<Property>,
// }
// struct Roomtype {
//     applicable_properties: Vec<Property>,
// }
// struct Boardingphase {
//     applicable_properties: Vec<Property>,
// }
// struct Boardingbehaviour {
//     applicable_properties: Vec<Property>,
// }
// struct Blacklisttype {
//     applicable_properties: Vec<Property>,
// }
// struct Blacklistgroup {
//     applicable_properties: Vec<Property>,
// }
// struct Waretransport {
//     applicable_properties: Vec<Property>,
// }
// struct Relationchangereason {
//     applicable_properties: Vec<Property>,
// }
// struct Shiptype {
//     applicable_properties: Vec<Property>,
// }
// struct Numeric {
//     applicable_properties: Vec<Property>,
// }
// struct Integer {
//     applicable_properties: Vec<Property>,
// }
// struct Boolean {
//     applicable_properties: Vec<Property>,
// }
// struct Money {
//     applicable_properties: Vec<Property>,
// }
// struct Largeint {
//     applicable_properties: Vec<Property>,
// }
// struct Float {
//     applicable_properties: Vec<Property>,
// }
// struct Length {
//     applicable_properties: Vec<Property>,
// }
// struct Angle {
//     applicable_properties: Vec<Property>,
// }
// struct Hitpoints {
//     applicable_properties: Vec<Property>,
// }
// struct Time {
//     applicable_properties: Vec<Property>,
// }
// struct Largefloat {
//     applicable_properties: Vec<Property>,
// }
