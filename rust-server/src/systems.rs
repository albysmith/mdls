use crate::*;
use specs::prelude::*;
// WELL, we currently cannot join against type... which ya know is the point of this
pub struct PrintNames;
impl<'a> System<'a> for PrintNames {
    // set the storages you want to use either ReadStorage or WriteStorage if you want to update them
    type SystemData = (Entities<'a>, ReadStorage<'a, NodeName>, ReadStorage<'a, MdEvents>);

    fn run(&mut self, (entities, node_storage, events): Self::SystemData) {
        for (node, ent, event) in (&node_storage, &entities, &events).join() {
            info!("Node Name: {:?} ent Name: {:?} event: {:?}", node, ent, event)
        }
        info!("node count{:?}", node_storage.count());
    }
}
pub struct PrintMe;
impl<'a> System<'a> for PrintMe {
    type SystemData = (ReadStorage<'a, Span>,);

    fn run(&mut self, (pos): Self::SystemData) {
        info!("span count: {:?}", pos.0.count())
    }
}

pub struct TypeAdder;
impl<'a> System<'a> for TypeAdder {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, NodeName>,
        WriteStorage<'a, MdEvents>,
        WriteStorage<'a, MdMethods>,
        Read<'a, EventList>,
        Read<'a, MethodList>,
    );

    fn run(
        &mut self,
        (entity, nodecomp, mut eventcomp, mut methcomp, eventlist, methodlist): Self::SystemData,
    ) {
        for node in (&entity, &nodecomp).join() {
            for event in eventlist.events.iter() {
                if event.id == (node.1).0 {
                    // info!("match ! {}, {}", event.id,(node.1).0);
                    let x = eventcomp.insert(
                        node.0,
                        MdEvents {
                            possible_types: event.clone(),
                        },
                    );
                    // info!("{:#?}", x);
                }
            }
            for method in methodlist.methods.iter() {
                if method.id == (node.1).0 {
                    let y = methcomp.insert(
                        node.0,
                        MdMethods {
                            possible_types: method.clone(),
                        },
                    );
                }
            }
        }
        info!("eventcomp count: {:#?}", eventcomp.count());
        info!("methcomp count: {:#?}", methcomp.count());
    }
}

pub struct MdEventsPrint;
impl<'a> System<'a> for MdEventsPrint {
    type SystemData = (ReadStorage<'a, MdEvents>, Entities<'a>);

    fn run(&mut self, (types, entities): Self::SystemData) {
        info!("mdtypes count: {:?}", types.count())
    }
}

pub struct MdMethodsPrint;
impl<'a> System<'a> for MdMethodsPrint {
    type SystemData = (ReadStorage<'a, MdMethods>, Entities<'a>);

    fn run(&mut self, (types, entities): Self::SystemData) {
        info!("mdtypes count: {:?}", types.count())
    }
}
