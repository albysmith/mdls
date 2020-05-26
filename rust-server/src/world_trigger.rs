use crate::error_handling::MDLSError;
use log::info;
use lsp_types::Url;
use specs::prelude::*;
use std::fs::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
// use crate::macros;

trait ComponentType {}
#[derive(Debug)]
pub struct NodeName(String);
impl Component for NodeName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for NodeName {}
#[derive(Debug)]
pub struct VariableName(String);
impl Component for VariableName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for VariableName {}
#[derive(Debug)]
pub struct CueName(String);
impl Component for CueName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for CueName {}
#[derive(Debug)]
pub struct ScriptName(String);
impl Component for ScriptName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for ScriptName {}

pub struct PrintNames;

impl<'a> System<'a> for PrintNames {
    // set the storages you want to use either ReadStorage or WriteStorage if you want to update them
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, NodeName>,
        ReadStorage<'a, ScriptName>,
    );

    fn run(&mut self, (entities, node_storage, script_storage): Self::SystemData) {
        for (node, script) in (&node_storage, &script_storage).join() {
            info!("Node Name: {:?} Script Name: {:?}", node, script)
        }
    }
}

pub fn parse_workspace(workspace_uri: Option<Url>) -> World {
    // this is probably the most disappointing code i've ever written.....
    let mut md_files: Vec<PathBuf> = vec![];
    if let Some(uri) = workspace_uri {
        if let Ok(path) = uri.to_file_path() {
            for entry in WalkDir::new(path) {
                if let Ok(f) = entry {
                    if !is_hidden(&f) {
                        if f.file_name() == "md" {
                            for md_file in WalkDir::new(f.path()) {
                                if let Ok(m) = md_file {
                                    if let Some(name) = m.file_name().to_str() {
                                        if name.contains(".xml") {
                                            md_files.push(m.into_path())
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    let world = world_trigger(md_files);
    world
}
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn world_trigger(md_files: Vec<PathBuf>) -> World {
    let mut world = World::new();
    world.register::<VariableName>();
    world.register::<CueName>();
    world.register::<NodeName>();
    world.register::<ScriptName>();

    let strings = md_files
        .iter()
        .map(|f| std::fs::read_to_string(&f))
        .filter_map(|s| s.ok())
        .collect::<Vec<_>>();
    for md_file in strings {
        if let Ok(doc) = roxmltree::Document::parse(&md_file) {
            parse_document(doc, &mut world);
        }
    }

    world
}

// needs to return Result/Error
fn parse_document(doc: roxmltree::Document, world: &mut World) {
    let mut mdscript_entity = world.create_entity().build();
    for node in doc.descendants() {
        // let mut entity = world
        //     .create_entity()
        // 	   .build();
        let node_name = node.tag_name().name();
        match node_name {
            "mdscript" => handle_mdscript_entity(node, mdscript_entity, world),
            "cues" => {}
            "cue" => {}
            "conditions" => {}
            "delay" => {}
            "actions" => {}
            "library" => {}
            "params" => {}
            "" => {}
            _ => {}
        }
    }
}

fn handle_mdscript_entity(node: roxmltree::Node, entity: Entity, world: &mut World) {
    let node_result = add_component!(
        NodeName,
        NodeName(node.tag_name().name().to_string()),
        entity,
        world
    );
    if let Some(script) = node.attribute("name") {
        let script_result =
            add_component!(ScriptName, ScriptName(script.to_string()), entity, world);
    }
}

// everything is an entity!!!
// smallest points possible for components; will use components to figure out what is what
// components needed:
//  - TextValue: every entity has a text value in the file | easily get from rox
//  - SpanInFile: every entity has a byte range in the file | easily get from rox
//  - FromFile: every entity comes from a particular file | easily get from file reading process
//  - HasNamespace: every entity has a namespace | should reference current cue entity that it exists within
//  - IsVariable: some entities will be variables; store relevant info on this struct |
//  - IsProperty: some entities will be scriptproperties; store in/out types on this struct |
//  - IsCue: some entities will be cues; store relevant info on this struct
//  - HasEventCondition: some entities will have event conditions; store relevant info on this struct
//  - HasParentCue (for namespace purposes): some entities will have parents, whose namespaces need to be included in this one
//  - IsRoot (for namespace purposes): some entities will be their own beginning namespace and won't need to include others

// I need to parse using rox to get all important info possible:
//  many match statements?  filter/map?
