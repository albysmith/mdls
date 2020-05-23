use log::info;
use lsp_types::Url;
use specs::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::*;

struct Namespace {
    variables: Vec<Variable>,
}

impl Component for Namespace {
    type Storage = VecStorage<Self>;
}
struct File {
    uri: Url,
}

impl Component for File {
    type Storage = VecStorage<Self>;
}

struct Variable {
    text: String,
    references: Vec<u32>,
    originator: Option<Vec<u32>>,
}

impl Component for Variable {
    type Storage = VecStorage<Self>;
}

struct Position {
    line: usize,
    character: usize,
}

struct Span {
    start: Position,
    end: Position,
}

impl Component for Span {
    type Storage = VecStorage<Self>;
}

// Entity is an instance of a variable somewhere in code
// Variable component tracks all instances of itself and tries to figure out its origin point
// Namespace tracks all the variables present in that namespace

pub fn parse_file(workspace_uri: Option<Url>) {
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
	info!("{:?}", md_files);
	
	for file in md_files {

	}

    let mut world = World::new();
    world.register::<Span>();
    world.register::<Variable>();
    world.register::<File>();
    world.register::<Namespace>();
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}


