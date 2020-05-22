use std::fs;
use std::iter;

pub struct ScriptProperties {
    properties: String,
}

impl ScriptProperties {
    pub fn new(path: &str) -> Self {
        ScriptProperties {
            properties: fs::read_to_string(path).expect("missing scriptproperties.xml path"),
        }
    }
}

struct Property {
    prop_name: String,
    prop_result: String,
    prop_type: String,
}

enum Datatypes {
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
}

impl iter::Iterator for Datatypes {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        use Datatypes::*;
        match self {
            Activity => Some(Adsign),
            Adsign => Some(Alertlevel),
            Alertlevel => Some(Angle),
            Angle => Some(Assignment),
            Assignment => Some(Attention),
            Attention => Some(Blacklistgroup),
            Blacklistgroup => Some(Blacklisttype),
            Blacklisttype => Some(Boardingbehaviour),
            Boardingbehaviour => Some(Boardingphase),
            Boardingphase => Some(Boolean),
            Boolean => Some(Build),
            Build => Some(Buildmodule),
            Buildmodule => Some(Buildprocessor),
            Buildprocessor => Some(Buildstorage),
            Buildstorage => Some(Cargolist),
            Cargolist => Some(Class),
            Class => Some(Cluster),
            Cluster => Some(Collectable),
            Collectable => Some(Command),
            Command => Some(Commandaction),
            Commandaction => Some(Component),
            Component => Some(Componentslot),
            Componentslot => Some(Componentstate),
            Componentstate => Some(Constructionplanentrydata),
            Constructionplanentrydata => Some(Constructionplanentryid),
            Constructionplanentryid => Some(Constructionsequence),
            Constructionsequence => Some(Container),
            Container => Some(Controllable),
            Controllable => Some(Controlpaneltype),
            Controlpaneltype => Some(Controlposition),
            Controlposition => Some(Controlpost),
            Controlpost => Some(Cue),
            Cue => Some(Cuestate),
            Cuestate => Some(Datatype),
            Datatype => Some(Dbdata),
            Dbdata => Some(Defensible),
            Defensible => Some(Deployablecategory),
            Deployablecategory => Some(Destructible),
            Destructible => Some(Dockarea),
            Dockarea => Some(Dockingbay),
            Dockingbay => Some(Dockstate),
            Dockstate => Some(Dronemode),
            Dronemode => Some(Drop),
            Drop => Some(Engine),
            Engine => Some(Entity),
            Entity => Some(Entityrole),
            Entityrole => Some(Entitytype),
            Entitytype => Some(Enum),
            Enum => Some(Explosive),
            Explosive => Some(Faction),
            Faction => Some(Flightbehaviour),
            Flightbehaviour => Some(Flightcontrolmodel),
            Flightcontrolmodel => Some(Float),
            Float => Some(Formationshape),
            Formationshape => Some(Galaxy),
            Galaxy => Some(Gate),
            Gate => Some(Group),
            Group => Some(Highway),
            Highway => Some(Highwayentrygate),
            Highwayentrygate => Some(Highwayexitgate),
            Highwayexitgate => Some(Hitpoints),
            Hitpoints => Some(Integer),
            Integer => Some(Killmethod),
            Killmethod => Some(Kwenum),
            Kwenum => Some(Largefloat),
            Largefloat => Some(Largeint),
            Largeint => Some(Length),
            Length => Some(Level),
            Level => Some(Licence),
            Licence => Some(List),
            List => Some(Loadout),
            Loadout => Some(Lock),
            Lock => Some(Lookuplist),
            Lookuplist => Some(Macro),
            Macro => Some(Missiongroup),
            Missiongroup => Some(Missiontype),
            Missiontype => Some(Module),
            Module => Some(Money),
            Money => Some(Moodlevel),
            Moodlevel => Some(Navcontext),
            Navcontext => Some(Nonplayer),
            Nonplayer => Some(Notification),
            Notification => Some(Npctemplate),
            Npctemplate => Some(Npctemplateentry),
            Npctemplateentry => Some(Numeric),
            Numeric => Some(Object),
            Object => Some(Objective),
            Objective => Some(Operation),
            Operation => Some(Order),
            Order => Some(Orderstate),
            Orderstate => Some(Pier),
            Pier => Some(Position),
            Position => Some(Purpose),
            Purpose => Some(Quadrant),
            Quadrant => Some(Race),
            Race => Some(Region),
            Region => Some(Relationchangereason),
            Relationchangereason => Some(Room),
            Room => Some(Roompopulationtype),
            Roompopulationtype => Some(Roomtype),
            Roomtype => Some(Rotation),
            Rotation => Some(Sector),
            Sector => Some(Shieldgenerator),
            Shieldgenerator => Some(Ship),
            Ship => Some(Shiptype),
            Shiptype => Some(Signalleaktype),
            Signalleaktype => Some(Skilltype),
            Skilltype => Some(Space),
            Space => Some(Spacesuit),
            Spacesuit => Some(Station),
            Station => Some(String),
            String => Some(Table),
            Table => Some(Tag),
            Tag => Some(Time),
            Time => Some(Trade),
            Trade => Some(Turret),
            Turret => Some(Unitcategory),
            Unitcategory => Some(Unlock),
            Unlock => Some(Vector),
            Vector => Some(Ventureplatform),
            Ventureplatform => Some(Walkablemodule),
            Walkablemodule => Some(Ware),
            Ware => Some(Wareamountlist),
            Wareamountlist => Some(Warelist),
            Warelist => Some(Waretransport),
            Waretransport => Some(Weapon),
            Weapon => Some(Weaponmode),
            Weaponmode => Some(Zone),
            Zone => None,
        }
    }
}

/*
    1)


*/

struct ApplicableProperties {
    datatype: Datatypes,
    properties: Vec<Property>,
}

impl ApplicableProperties {
    fn search(&self, string: &str) -> Option<&Property> {
        for prop in self.properties.iter() {
            if string.to_string() == prop.prop_name {
                return Some(prop)
            }
        }
        None
    }
}

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
