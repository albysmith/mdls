use crate::*;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Variable {
    pub value: String,
    pub script: Option<Entity>,
    pub cue: Option<Entity>,

    pub node: Option<Entity>,
    pub name: String,

    pub possible_types: Vec<Datatypes>,
    pub path: String,
}
#[derive(Debug, Default, Clone)]
pub struct Node {
    pub value: String,
    pub script: Option<Entity>,
    pub cue: Option<Entity>,
    pub path: String,
    pub event: Option<Event>,
    pub method: Option<Method>,
    pub variables: Vec<Entity>,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cue {
    pub value: String,
    pub script: Option<Entity>,
    pub parent: Option<Entity>,
    pub child: Option<Entity>,
    pub new: bool,
    pub nodes: Vec<Entity>,
    // pub variables: Vec<Entity>,
    pub path: MdPath,
    pub newspace: bool,
    pub spath: String,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Script {
    pub value: String,
    pub cues: Vec<Entity>,
    pub path: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MdPath {
    pub script: Option<Entity>,
    pub cue: Option<Entity>,
}
impl Component for Variable {
    type Storage = VecStorage<Self>;
}
impl Component for Node {
    type Storage = VecStorage<Self>;
}
impl Component for Cue {
    type Storage = VecStorage<Self>;
}
impl Component for Script {
    type Storage = VecStorage<Self>;
}
impl Component for MdPath {
    type Storage = VecStorage<Self>;
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Buffy {
    pub script: Option<Entity>,
    pub cue: Option<Entity>,
    pub node: Option<Entity>,
    pub variable: Option<Entity>,
    pub parent: Option<Entity>,
    pub child: Option<Entity>,
    pub is_new: bool,
    pub is_md: bool,
    pub is_library: bool,
}

impl Buffy {
    pub fn next(&mut self) {
        self.script = None;
        self.cue = None;
        self.node = None;
        self.variable = None;
    }
}
