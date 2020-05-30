use crate::*;
pub struct PrintNames;
impl<'a> System<'a> for PrintNames {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, NodeName>,
        ReadStorage<'a, MdEvents>,
    );
    fn run(&mut self, (entities, node_storage, events): Self::SystemData) {
        for (node, ent, event) in (&node_storage, &entities, &events).join() {
            info!(
                "Node Name: {:?} ent Name: {:?} event: {:?}",
                node, ent, event
            )
        }
        info!("node count{:?}", node_storage.count());
    }
}

pub struct EventAdder;
impl<'a> System<'a> for EventAdder {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, NodeName>,
        WriteStorage<'a, MdEvents>,
        Read<'a, EventList>,
    );
    fn run(&mut self, (entity, nodecomp, mut eventcomp, eventlist): Self::SystemData) {
        for node in (&entity, &nodecomp).join() {
            for event in eventlist.events.iter() {
                if event.id == (node.1).0 {
                    let _x = eventcomp.insert(
                        node.0,
                        MdEvents {
                            possible_types: event.clone(),
                        },
                    );
                }
            }
        }
        info!("events");
    }
}
pub struct MethodAdder;
impl<'a> System<'a> for MethodAdder {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, NodeName>,
        WriteStorage<'a, MdMethods>,
        Read<'a, MethodList>,
    );
    fn run(&mut self, (entity, nodecomp, mut methcomp, methodlist): Self::SystemData) {
        for node in (&entity, &nodecomp).join() {
            for method in methodlist.methods.iter() {
                if method.id == (node.1).0 {
                    let _y = methcomp.insert(
                        node.0,
                        MdMethods {
                            possible_types: method.clone(),
                        },
                    );
                }
            }
        }
        info!("methods");
    }
}

// pub struct TypeVar;
// impl<'a> System<'a> for TypeVar {
//     type SystemData = (
//         Entities<'a>,
//         ReadStorage<'a, NodeName>,
//         WriteStorage<'a, MdMethods>,
//         Read<'a, MethodList>,
//     );
//     fn run(&mut self, (entity, nodecomp, mut methcomp, methodlist): Self::SystemData) {
//         for node in (&entity, &nodecomp).join() {
//             for method in methodlist.methods.iter() {
//                 if method.id == (node.1).0 {
//                     let _y = methcomp.insert(
//                         node.0,
//                         MdMethods {
//                             possible_types: method.clone(),
//                         },
//                     );
//                 }
//             }
//         }
//         info!("methods");
//     }
// }

pub struct MdEventsPrint;
impl<'a> System<'a> for MdEventsPrint {
    type SystemData = ReadStorage<'a, MdEvents>;

    fn run(&mut self, types: Self::SystemData) {
        info!("events count: {:?}", types.count())
    }
}

pub struct MdMethodsPrint;
impl<'a> System<'a> for MdMethodsPrint {
    type SystemData = ReadStorage<'a, MdMethods>;

    fn run(&mut self, types: Self::SystemData) {
        info!("methods count: {:?}", types.count())
    }
}

pub struct PrintMe;
impl<'a> System<'a> for PrintMe {
    type SystemData = ReadStorage<'a, Span>;

    fn run(&mut self, pos: Self::SystemData) {
        info!("span count: {:?}", pos.count())
    }
}

pub struct PrintGraph;
impl<'a> System<'a> for PrintGraph {
    type SystemData = (
        ReadStorage<'a, components::Script>,
        ReadStorage<'a, components::Cue>,
        ReadStorage<'a, components::Node>,
        ReadStorage<'a, components::Variable>,
    );

    fn run(&mut self, (script, cue, node, var): Self::SystemData) {
        info!("span count: {:?}", script.count());
        // for s in script.join() {
        //     info!("{:#?}", s)
        // }
        // for c in cue.join() {
        //     info!("{:#?}", c)
        // }
        // for n in node.join() {
        //     info!("{:#?}", n)
        // }
        // for v in var.join() {
        //     info!("{:#?}", v)
        // }
    }
}

pub struct GraphTypingMethods;
impl<'a> System<'a> for GraphTypingMethods {
    type SystemData = (WriteStorage<'a, components::Node>, Read<'a, MethodList>);

    fn run(&mut self, (mut node, methodlist): Self::SystemData) {
        for node in (&mut node).join() {
            for method in methodlist.methods.iter() {
                if node.value == method.id {
                    node.method = Some((*method).clone())
                }
            }
        }
    }
}
pub struct GraphTypingEvents;
impl<'a> System<'a> for GraphTypingEvents {
    type SystemData = (WriteStorage<'a, components::Node>, Read<'a, EventList>);

    fn run(&mut self, (mut node, eventlist): Self::SystemData) {
        for node in (&mut node).join() {
            for method in eventlist.events.iter() {
                if node.value == method.id {
                    node.event = Some((*method).clone())
                }
            }
        }
    }
}
