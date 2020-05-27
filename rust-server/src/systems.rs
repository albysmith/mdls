use crate::*;
use specs::prelude::*;

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
        info!("script storage count{:?}", script_storage.count());
        info!("entities count{:?}", node_storage.count())
    }
}
pub struct PrintMe;
impl<'a> System<'a> for PrintMe {
    type SystemData = (ReadStorage<'a, Span>,);

    fn run(&mut self, (pos): Self::SystemData) {
        info!("{:?}", pos.0.count())
    }
}

pub struct TypeAdder;
impl<'a> System<'a> for TypeAdder {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, NodeName>,
        WriteStorage<'a, MdTypes>,
        Read<'a, EventList>,
        Read<'a, MethodList>,
    );

    fn run(&mut self, (entity, nodecomp, mut typecomp, eventlist, methodlist): Self::SystemData) {
        info!("{:?}", nodecomp.count());
        for node in (&entity, &nodecomp).join() {
            for event in eventlist.events.iter() {
                if event.id == (node.1).0 {
                    // info!("match ! {}, {}", event.id,(node.1).0);
                    let x = typecomp.insert(
                        node.0,
                        MdTypes {
                            possible_types: event.clone(),
                        },
                    );
                    // info!("{:#?}", x);
                }
            }
        }
        info!("TYPECOMP COUNT{:#?}", typecomp.count());
    }
}

pub struct MdTypesPrint;
impl<'a> System<'a> for MdTypesPrint {
    type SystemData = (ReadStorage<'a, MdTypes>,);

    fn run(&mut self, (types): Self::SystemData) {
        info!("mdtypes.count: {:?}", types.0.count())
    }
}
