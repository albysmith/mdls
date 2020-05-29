use crate::*;
use log::info;
use lsp_types::Url;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

trait ComponentType {
    fn create_component<T: Component>(x: T, world: &mut World, entitiy: Entity) {
        let _x = world.write_component::<T>().insert(entitiy, x);
        // info!("create_component: happened");
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
    pub start: Position,
    pub end: Position,
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
    world
}

fn create_world() -> World {
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
                start: Position {
                    bytes: start_pos as usize,
                },
                end: Position {
                    bytes: end_pos as usize,
                },
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
                                // let start_pos = doc.text_pos_at(attr.value_range().start);
                                // let end_pos = doc.text_pos_at(attr.value_range().end);
                                let start_pos = attr.value_range().start;
                                let end_pos = attr.value_range().end;
                                world
                                    .create_entity()
                                    .with(Span {
                                        start: Position {
                                            bytes: start_pos as usize,
                                        },
                                        end: Position {
                                            bytes: end_pos as usize,
                                        },
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

    if let Some(files) = files {
        for file in files.iter() {
            if let Ok(string) = fs::read_to_string(&file.path) {
                if let Ok(doc) = roxmltree::Document::parse(&string) {
                    parse_doc_to_components(doc, &mut world)
                }
            }
        }
    }
    world
}
// here for reference
// pub struct Buffy {
//     pub script: i32,
//     pub cue: Vec<i32>,
//     pub node: Vec<i32>,
//     pub namespace: Vec<i32>,
//     pub variable: Vec<i32>,
//     pub this_flag: bool,
// }
// TODO: UNFUCK THE UNWRAPS
fn parse_doc_to_components(doc: roxmltree::Document, world: &mut World) {
    use components::*;
    let mut buffy = components::Buffy::default();
    buffy.namespace = 1;
    for node in doc.descendants() {
        let node_name = node.tag_name().name();
        match node_name {
            "mdscript" => {
                buffy.next();
                buffy.script = Some(
                    world
                        .create_entity()
                        .with(components::Script {
                            cues: vec![],
                            path: "TEMPpath".to_string(),
                            value: node.attribute("name").unwrap().to_owned(),
                        })
                        .build(),
                );
            }
            "cues" => {}
            "cue" => {
                if let Some(namespace) = node.attribute("namespace") {
                    if namespace == "this" {
                        // TODO FILL BUFFY ENTS, NEXT  UPDATES

                        world.maintain();

                        let mut script_storage = world.write_storage::<components::Script>();
                        let mut cue_storage = world.write_storage::<components::Cue>();
                        let mut node_storage = world.write_storage::<components::Node>();
                        let mut variable_storage = world.write_storage::<components::Variable>();
                        for cue in buffy.cue.to_owned().into_iter() {
                            if let Some(cue_comp) = cue_storage.get_mut(cue) {
                                for bnode in buffy.node.iter() {
                                    let x = node_storage.get_mut(bnode.to_owned()).unwrap();
                                    info!("CUE!!!!{:#?} == {:#?}", x.cue, Some(cue));
                                    if x.cue == Some(cue) {
                                        info!("CUE MATCHED!!!!!!");
                                        cue_comp.nodes.push(cue)
                                    }
                                    for var in buffy.variable.iter() {
                                        if let Some(v) = variable_storage.get(var.to_owned()) {
                                    info!("node !!!!{:#?} == {:#?}", v.node, Some(bnode.to_owned()));

                                            if v.node == Some(bnode.to_owned()) {
                                                info!("node MATCHED!!!!!!");

                                                cue_comp.variables.push(var.to_owned());
                                                x.variables.push(var.to_owned())
                                            }
                                        }
                                    }
                                }
                            }
                            // buffer needs to store a nested object of cue/node/val
                        }
                        buffy.this_flag = true;
                    }
                };
                // if someshit is a new namespace {
                //probably
                // }
                buffy.cue.push(
                    world
                        .create_entity()
                        .with(Cue {
                            script: buffy.script,
                            namespace: buffy.namespace,
                            nodes: vec![],
                            variables: vec![],
                            value: node.attribute("name").unwrap().to_owned(),
                            path: MdPath {
                                script: buffy.script,
                                cue: None,
                            },
                            newspace: false,
                        })
                        .build(),
                )
            }
            "conditions" => {}
            "delay" => {}
            "actions" => {}
            "library" => {}
            "params" => {}
            "" => {}
            _ => {
                buffy.node.push(
                    world
                        .create_entity()
                        .with(Node {
                            script: buffy.script,
                            cue: Some(buffy.cue.last().unwrap().to_owned()),
                            namespace: buffy.namespace,
                            value: node_name.to_owned(),
                            event: None,
                            method: None,
                            ..Default::default()
                        })
                        .build(),
                );
                for attr in node.attributes() {
                    buffy.variable.push(
                        world
                            .create_entity()
                            .with(components::Variable {
                                script: buffy.script,
                                cue: Some(buffy.cue.last().unwrap().to_owned()),
                                namespace: buffy.namespace,
                                node: Some(buffy.node.last().unwrap().to_owned()),
                                value: attr.value().to_owned(),
                                name: attr.name().to_owned(),
                                ..Default::default()
                            })
                            .build(),
                    )
                }
            }
        }
    }
}
