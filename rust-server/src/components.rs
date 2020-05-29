use crate::*;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Variable {
    pub value: String,
    pub script: Option<Entity>,
    pub cue: Option<Entity>,
    pub namespace: i32,
    pub node: Option<Entity>,
    pub name: String,
    
    pub possible_types: Vec<Datatypes>,
}
#[derive(Debug, Default, Clone)]
pub struct Node {
    pub value: String,
    pub script: Option<Entity>,
    pub cue: Option<Entity>,
    pub namespace: i32,
    pub event: Option<Event>,
    pub method: Option<Method>,
    pub variables: Vec<Entity>,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cue {
    pub value: String,
    pub script: Option<Entity>,
    pub namespace: i32,
    pub nodes: Vec<Entity>,
    // pub variables: Vec<Entity>,
    pub path: MdPath,
    pub newspace: bool,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Script {
    pub value: String,
    pub cues: Vec<Entity>,
    pub path: String,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NameSpace {
    pub cues: Vec<Entity>,
    pub vars: Vec<Entity>,
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
impl Component for NameSpace {
    type Storage = VecStorage<Self>;
}
impl Component for MdPath {
    type Storage = VecStorage<Self>;
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Buffy {
    pub script: Option<Entity>,
    pub cue: Vec<Entity>,
    pub node: Vec<Entity>,
    pub namespace: Vec<i32>,
    pub variable: Vec<Entity>,
    pub this_flag: bool,
    pub reset: bool,
}

impl Buffy {
    pub fn next(&mut self) {
        self.script = None;
        self.cue.clear();
        self.namespace.clear();
        self.node.clear();
        self.variable.clear();
        self.reset = false;
        self.this_flag = false;
    }
}
