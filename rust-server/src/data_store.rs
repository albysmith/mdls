use log::info;
use lsp_types::Url;
use specs::prelude::*;
use std::fs::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

struct Namespace {
    variables: Vec<Variable>,
}

impl Component for Namespace {
    type Storage = VecStorage<Self>;
}
pub struct File {
    pub uri: Url,
    pub path: PathBuf,
}

impl Component for File {
    type Storage = VecStorage<Self>;
}

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

pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Component for Span {
    type Storage = VecStorage<Self>;
}

// struct RoxNode<'a, 'input> {
// 	node: roxmltree::Node<'a, 'input>
// }

// impl Component for RoxNode {
//     type Storage = VecStorage<Self>;
// }

// Entity is an instance of a variable somewhere in code
// Variable component tracks all instances of itself and tries to figure out its origin point
// Namespace tracks all the variables present in that namespace

pub fn parse_file(workspace_uri: Option<Url>) -> World {
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
    // info!("{:?}", md_files);

    //NEW WAY OF CREATING ECS
    // let world = world_trigger(md_files);
    //END NEW WAY OF CREATING ECS

    //OLD WAY OF CREATING ECS
    let mut world = World::new();
    world.register::<Span>();
    world.register::<Variable>();
    world.register::<File>();
    world.register::<Namespace>();

    for file in md_files {
        if let Ok(file_uri) = Url::from_file_path(&file) {
            if let Ok(string) = std::fs::read_to_string(&file) {
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
                                        uri: file_uri.clone(),
                                        path: file.clone(),
                                    })
                                    .build();
                            }
                        }
                    }
                }
            }
        }
    }
    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintMe, "printme", &[])
        .build();
    dispatcher.dispatch(&mut world);
    // END OLD WAY OF ECS

    world
}

struct PrintMe;
impl<'a> System<'a> for PrintMe {
    type SystemData = (ReadStorage<'a, Span>,);

    fn run(&mut self, (pos): Self::SystemData) {
        info!("{:?}", pos.0.count())
    }
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// pub struct GoToDef;
// impl<'a> System<'a> for GoToDef {
//     type SystemData = (
//         ReadStorage<'a, Span>,
//         ReadStorage<'a, File>,
//         ReadStorage<'a, Variable>,
//     );

//     fn run(&mut self, (span_storage, uri_storage, variable_storage): Self::SystemData) {
//         for (span, uri, variable) in (&span_storage, &uri_storage, &variable_storage).join() {}
//     }
// }

// #[derive(SystemData)]
// pub struct ECSSystemData<'a> {
//     span: ReadStorage<'a, Span>,
//     uri: ReadStorage<'a, File>,
//     variable: ReadStorage<'a, Variable>,
// }
