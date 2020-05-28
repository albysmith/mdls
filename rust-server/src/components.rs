use crate::*;
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct Variable {
    pub script: u32,
    pub cue: u32,
    pub namespace: u32,
    pub node: u32,
    pub value: String,
    pub name: String,

    pub possible_types: Vec<Datatypes>,
}
#[derive(Default, Debug, Clone)]
pub struct Node {
    pub script: u32,
    pub cue: u32,
    pub namespace: u32,
    pub value: String,
    pub event: Option<Event>,
    pub method: Option<Method>,
}
#[derive(Default, Debug, Clone)]
pub struct Cue {
    pub script: u32,
    pub namespace: u32,
    pub nodes: Vec<u32>,
    pub value: String,
    pub path: MdPath,
    pub newspace: bool,
}
#[derive(Default, Debug, Clone)]
pub struct Script {
    pub cues: Vec<u32>,
    pub value: String,
    pub path: String,
}
#[derive(Default, Debug, Clone)]
pub struct NameSpace {
    pub cues: Vec<u32>,
    pub vars: Vec<u32>,
}
#[derive(Default, Debug, Clone)]
pub struct MdPath {
    pub script: u32,
    pub cue: u32,
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
#[derive(Default, Debug, Clone)]
pub struct Buffy {
    pub script: u32,
    pub cue: Vec<u32>,
    pub node: Vec<u32>,
    pub namespace: u32,
    pub variable: Vec<u32>,
    pub this_flag: bool,
    pub reset: bool,
}
impl Buffy {
    pub fn reset(&mut self) {
        self.cue.clear();
        self.namespace = 1;
        self.node.clear();
        self.script = 1;
        self.variable.clear();
        self.reset = false;
        self.this_flag = false;
    
    } 
}