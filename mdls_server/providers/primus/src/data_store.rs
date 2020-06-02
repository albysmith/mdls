use crate::*;
use log::info;
use lsp_types::Url;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

trait ComponentType {
    fn create_component<T: Component>(x: T, world: &mut World, entity: Entity) {
        let _x = world.write_component::<T>().insert(entity, x);
    }
}

#[derive(Default, Debug, Clone)]
pub struct MdMethods {
    pub possible_types: Method,
}
impl Component for MdMethods {
    type Storage = VecStorage<Self>;
}
#[derive(Default, Debug, Clone)]
pub struct MdEvents {
    pub possible_types: Event,
}
impl Component for MdEvents {
    type Storage = VecStorage<Self>;
}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct NodeName(pub String);
impl Component for NodeName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for NodeName {}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct VariableName(pub String);
impl Component for VariableName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for VariableName {}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct CueName(pub String);
impl Component for CueName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for CueName {}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct ScriptName(pub String);
impl Component for ScriptName {
    type Storage = VecStorage<Self>;
}
impl ComponentType for ScriptName {}
#[derive(Default, Debug, Clone)]
struct Namespace {
    variables: Vec<Variable>,
}
impl Component for Namespace {
    type Storage = VecStorage<Self>;
}
#[derive(Debug, Clone)]
pub struct File {
    pub uri: Url,
    pub path: PathBuf,
}
impl Component for File {
    type Storage = VecStorage<Self>;
}
#[derive(Default, Debug, Clone)]
pub struct Variable {
    pub text: String,
    pub references: Vec<u32>,
    pub originator: Option<Vec<u32>>,
}
impl Component for Variable {
    type Storage = VecStorage<Self>;
}
#[derive(Default, Debug, Clone, Copy)]
pub struct Position {
    // line: usize,
    // character: usize,
    pub bytes: usize,
}
#[derive(Default, Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
impl Component for Span {
    type Storage = VecStorage<Self>;
}

pub fn generate_world(workspace_uri: Option<Url>) -> World {
    let files = get_xml(workspace_uri);

    let mut world = create_world();
    go_to_def_temp(&files, &mut world);

    if let Some(files) = files {
        for file in files.iter() {
            if let Ok(string) = fs::read_to_string(&file.path) {
                if let Ok(doc) = roxmltree::Document::parse(&string) {
                    info!("generate_world: {}", string.len());
                    parse_document(doc, &mut world)
                }
            }
        }
    }
    info!("world generated");
    world
}

pub fn create_world() -> World {
    let mut world = World::new();
    world.register::<Span>();
    world.register::<Variable>();
    world.register::<File>();
    world.register::<Namespace>();
    world.register::<NodeName>();
    world.register::<VariableName>();
    world.register::<CueName>();
    world.register::<ScriptName>();
    world.register::<MdEvents>();
    world.register::<MdMethods>();
    world.register::<components::Script>();
    world.register::<components::Cue>();
    world.register::<components::Node>();
    world.register::<components::Variable>();
    world.register::<components::ParsedExp>();
    world
}

pub fn get_xml(workspace_uri: Option<Url>) -> Option<Vec<File>> {
    if let Some(url) = workspace_uri {
        if let Ok(path) = url.to_file_path() {
            let xml = WalkDir::new(path)
                .into_iter()
                .filter_map(|pr| pr.ok())
                .filter(|p| {
                    p.path()
                        .extension()
                        .filter(|e| e.to_str() == Some("xml"))
                        .is_some()
                })
                .map(|d| d.into_path())
                .filter(|p| {
                    for comp in p.as_path().components() {
                        if let Some(word) = comp.as_os_str().to_str() {
                            match word {
                                "cutscenes" => return false,
                                "assets" => return false,
                                "index" => return false,
                                "libraries" => return false,
                                "maps" => return false,
                                "music" => return false,
                                "particles" => return false,
                                "sfx" => return false,
                                "shadergl" => return false,
                                "t" => return false,
                                "textures" => return false,
                                "ui" => return false,
                                "voice-l044" => return false,
                                "voice-l049" => return false,
                                "vulkan" => return false,
                                _ => (),
                            }
                        }
                    }
                    true
                })
                .map(|p| File {
                    uri: url.clone(),
                    path: p,
                })
                .collect::<Vec<File>>();

            return Some(xml);
        }
    }
    None
}

fn parse_document(doc: roxmltree::Document, world: &mut World) {
    let mdscript_entity = world.create_entity().build();
    world.maintain();
    for node in doc.descendants() {
        let node_name = node.tag_name().name();
        match node_name {
            "mdscript" => <ScriptName as ComponentType>::create_component(
                ScriptName(node_name.to_string()),
                world,
                mdscript_entity,
            ),
            "cues" => {}
            "cue" => {
                let cue_entity = world.create_entity().build();
                <CueName as ComponentType>::create_component(
                    CueName(node_name.to_string()),
                    world,
                    cue_entity,
                )
            }
            "conditions" => {}
            "delay" => {}
            "actions" => {}
            "library" => {}
            "params" => {}
            "" => {}
            _ => {
                let node_entity = world.create_entity().build();
                <NodeName as ComponentType>::create_component(
                    NodeName(node_name.to_string()),
                    world,
                    node_entity,
                );
                attr_parse(node, world)
            }
        }
    }
}

pub fn attr_parse(node: roxmltree::Node, world: &mut World) {
    for attr in node.attributes() {
        let start_pos = attr.value_range().start;
        let end_pos = attr.value_range().end;
        world
            .create_entity()
            .with(Span {
                start: start_pos as usize,
                end: end_pos as usize,
            })
            .with(Variable {
                text: attr.value().to_string(),
                references: vec![],
                originator: None,
            })
            .with(VariableName(node.tag_name().name().to_owned()))
            .build();
    }
}

pub fn go_to_def_temp(paths: &Option<Vec<File>>, world: &mut World) {
    if let Some(files) = paths {
        for file in files.iter() {
            if let Ok(string) = std::fs::read_to_string(&file.path) {
                info!("go_to_def_temp: {}", string.len());

                if let Ok(doc) = roxmltree::Document::parse(&string) {
                    for node in doc.descendants() {
                        for attr in node.attributes() {
                            if attr.name() == "name" {
                                let start_pos = attr.value_range().start;
                                let end_pos = attr.value_range().end;
                                world
                                    .create_entity()
                                    .with(Span {
                                        start: start_pos as usize,
                                        end: end_pos as usize,
                                    })
                                    .with(Variable {
                                        text: attr.value().to_string(),
                                        references: vec![],
                                        originator: None,
                                    })
                                    .with(File {
                                        uri: file.uri.clone(),
                                        path: file.path.clone(),
                                    })
                                    .build();
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn new_generate_world(workspace_uri: Option<Url>) -> World {
    let files = get_xml(workspace_uri);
    let mut world = create_world();
    if let Some(files1) = files {
        for file in files1.iter() {
            if let Ok(string) = fs::read_to_string(&file.path) {
                if let Ok(doc) = roxmltree::Document::parse(&string) {
                    parse_doc(doc, &mut world, file.path.to_str().unwrap().to_string())
                }
            }
        }
    }
    info!("world generated");
    world
}

fn parse_doc_to_components(doc: roxmltree::Document, world: &mut World) {
    use components::*;
    let mut buffy = components::Buffy::default();
    for node in doc.descendants() {
        let node_name = node.tag_name().name();
        match node_name {
            "mdscript" => {
                buffy.is_md = true;
                buffy.next();
                buffy.script = Some(
                    world
                        .create_entity()
                        .with(components::Script {
                            path: "TEMPpath".to_string(),
                            value: node.attribute("name").expect("script name").to_owned(),
                            ..Default::default()
                        })
                        .build(),
                );
            }
            "cues" => {
                if buffy.is_md == true {
                    buffy.parent = buffy.cue
                }
            }
            "cue" => {
                if buffy.is_library == false && buffy.is_md == true {
                    if let Some(namespace_flag) = node.attribute("namespace") {
                        if namespace_flag == "this" {
                            buffy.is_new = true;
                        } else {
                            buffy.is_new = false;
                        }
                    } else {
                        buffy.is_new = false;
                    }

                    let this_cue = world
                        .create_entity()
                        .with(Cue {
                            script: buffy.script,
                            value: node.attribute("name").expect("cue name").to_owned(),
                            new: buffy.is_new,
                            parent: buffy.parent,
                            child: buffy.child,
                            path: components::MdPath {
                                script: buffy.script,
                                cue: None,
                            },
                            newspace: false,
                            ..Default::default()
                        })
                        .build();
                    buffy.cue = Some(this_cue);
                    if let Some(child) = node.first_child() {
                        if child.tag_name().name() != "cues" {}
                    }
                }
            }
            "conditions" => {}
            "delay" => {}
            "actions" => {}
            "library" => buffy.is_library = true,
            "params" => {}
            "param" => {}
            "" => {}
            _ => {
                if buffy.is_library == false && buffy.is_md == true {
                    let this_node = world
                        .create_entity()
                        .with(Node {
                            script: buffy.script,
                            cue: buffy.cue,
                            value: node_name.to_owned(),
                            event: None,
                            method: None,
                            ..Default::default()
                        })
                        .build();

                    for attr in node.attributes() {
                        let _this_var = world
                            .create_entity()
                            .with(components::Variable {
                                script: buffy.script,
                                cue: buffy.cue,
                                node: Some(this_node),
                                value: attr.value().to_owned(),
                                name: attr.name().to_owned(),
                                ..Default::default()
                            })
                            .build();
                    }
                }
            }
        }
    }
}

// NEW WAY BELOW HERE
struct ParentInfo {
    script: Entity,
    path: String,
    cue: Option<Entity>,
}

pub fn parse_doc(doc: roxmltree::Document, world: &mut World, path: String) {
    for mdscript in doc.root().children() {
        if mdscript.tag_name().name() == "mdscript" {
            let script = world
                .create_entity()
                .with(components::Script {
                    path: path.to_owned(),
                    value: mdscript.attribute("name").expect("script name").to_owned(),
                    ..Default::default()
                })
                .build();
            let parent = ParentInfo {
                script: script,
                path: path.to_owned(),
                cue: None,
            };
            for child in mdscript.children() {
                match child.tag_name().name() {
                    "cues" => {
                        for cue in child.children() {
                            match cue.tag_name().name() {
                                "cue" => process_cue(cue, &parent, world),
                                "library" => process_library(cue, &parent, world),
                                _ => {}
                            }
                        }
                    }
                    "" => {}
                    _ => {}
                }
            }
        }
    }
}

fn process_cue(cue: roxmltree::Node, parent: &ParentInfo, world: &mut World) {
    let mut newspace = false;
    if let Some(this) = cue.attribute("namespace") {
        if this == "this" {
            newspace = true
        }
    }
    let cue_entity = world
        .create_entity()
        .with(components::Cue {
            script: Some(parent.script),
            value: cue.attribute("name").expect("cue name").to_owned(),
            new: true,
            parent: if newspace == false { parent.cue } else { None },
            path: components::MdPath {
                script: Some(parent.script),
                cue: parent.cue,
            },
            spath: parent.path.to_owned(),
            newspace: newspace,
            ..Default::default()
        })
        .build();
    let cue_parent = ParentInfo {
        script: parent.script,
        path: parent.path.to_owned(),
        cue: Some(cue_entity),
    };
    for node in cue.children() {
        match node.tag_name().name() {
            "conditions" => process_nodes(node, &cue_parent, world),
            "delay" => process_delay(node, &cue_parent, world),
            "actions" => process_nodes(node, &cue_parent, world),
            "patch" => process_nodes(node, &cue_parent, world),
            "cues" => {
                for cue in node.children() {
                    match cue.tag_name().name() {
                        "cue" => process_cue(cue, &cue_parent, world),
                        "library" => process_library(cue, &cue_parent, world),
                        _ => (),
                    }
                }
            }
            _ => {}
        }
    }
}
fn process_library(library: roxmltree::Node, parent: &ParentInfo, world: &mut World) {
    for node in library.children() {
        match node.tag_name().name() {
            "conditions" => {}
            "params" => {}
            "delay" => {}
            "actions" => {}
            "cues" => {
                for cue in node.children() {
                    match cue.tag_name().name() {
                        "cue" => {}
                        "library" => {}
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn process_nodes(nodes: roxmltree::Node, parent: &ParentInfo, world: &mut World) {
    for node in nodes.descendants() {
        match node.tag_name().name() {
            "" => {}
            " " => {}
            "actions" => {}
            "conditions" => {}
            "patch" => {}
            _ => {
                let this_node = world
                    .create_entity()
                    .with(components::Node {
                        script: Some(parent.script),
                        cue: parent.cue,
                        value: node.tag_name().name().to_owned(),
                        event: None,
                        method: None,
                        path: parent.path.to_owned(),
                        ..Default::default()
                    })
                    .build();
                for attr in node.attributes() {
                    let _this_var = world
                        .create_entity()
                        .with(components::Variable {
                            script: Some(parent.script),
                            cue: parent.cue,
                            node: Some(this_node),
                            value: attr.value().to_owned(),
                            name: attr.name().to_owned(),
                            path: parent.path.to_owned(),
                            ..Default::default()
                        })
                        .with(Span {
                            start: attr.value_range().start,
                            end: attr.value_range().end,
                        })
                        .build();
                }
            }
        }
    }
}
fn process_delay(delay: roxmltree::Node, parent: &ParentInfo, world: &mut World) {}
