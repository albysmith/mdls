use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;

fn main() {
    //   let pretty = PrettyConfig::new()
    //     .with_depth_limit(2)
    //     .with_separate_tuple_members(true)
    //     .with_enumerate_arrays(true);
    // let s = to_string_pretty(&data, pretty).expect("Serialization failed");
    // println!("Hello, world!");

    generate_structs()
}

struct PropRustSource {
    rust_source: String,
}
impl PropRustSource {
    fn new() -> Self {
        PropRustSource {
            rust_source: String::new(),
        }
    }
    fn datatype(&mut self, name: &str) {
        self.rust_source
            .push_str(&format!("\nstruct {} {{", name.to_owned(),))
    }
    fn property(&mut self, name: &str) {
        self.rust_source.push_str(&format!("\n {}:String,\n", name))
    }
    fn close(&mut self) {
        self.rust_source.push_str("}")
    }
}

fn generate_structs() {
    let string = include_str!("reference/scriptproperties.xml");
    let doc = roxmltree::Document::parse(string).expect("malformed xml i guess");

    let mut struct_source = PropRustSource::new();
    let mut flag = false;
    for node in doc.descendants() {
        match node.tag_name().name() {
            
            "datatype" | "keyword" => {
                if flag {
                    flag = false;
                    struct_source.close()
                } else {
                }
                struct_source.datatype(node.attribute("name").unwrap());
            }

            "property" => {
                struct_source.property(node.attribute("name").unwrap());
                flag = true;
            }
            _ => (),
        }
    }
    println!("{}", struct_source.rust_source);
}
