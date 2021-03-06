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

pub struct AddVarsToNodes;
impl<'a> System<'a> for AddVarsToNodes {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::Node>,
        ReadStorage<'a, components::Variable>,
    );

    fn run(&mut self, (entities, mut node_storage, var_storage): Self::SystemData) {
        for (var, entity) in (&var_storage, &entities).join() {
            if let Some(node) = node_storage.get_mut(var.node.unwrap()) {
                node.variables.push(entity)
            }
        }
        info!("vars added to node components")
    }
}
pub struct AddVarsToCues;
impl<'a> System<'a> for AddVarsToCues {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::Cue>,
        ReadStorage<'a, components::Variable>,
    );

    fn run(&mut self, (entities, mut cue_storage, var_storage): Self::SystemData) {
        for (var, entity) in (&var_storage, &entities).join() {
            if let Some(cue) = cue_storage.get_mut(var.cue.unwrap()) {
                cue.variables.push(entity)
            }
        }
        info!("vars added to cue components")
    }
}
pub struct AddNodesToCues;
impl<'a> System<'a> for AddNodesToCues {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::Cue>,
        ReadStorage<'a, components::Node>,
    );

    fn run(&mut self, (entities, mut cue_storage, node_storage): Self::SystemData) {
        for (node, entity) in (&node_storage, &entities).join() {
            if let Some(cue) = cue_storage.get_mut(node.cue.unwrap()) {
                cue.nodes.push(entity)
            }
        }
        info!("nodes added to cue components")
    }
}
pub struct AddCuesToScript;
impl<'a> System<'a> for AddCuesToScript {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::Script>,
        ReadStorage<'a, components::Cue>,
    );

    fn run(&mut self, (entities, mut script_storage, cue_storage): Self::SystemData) {
        for (cue, entity) in (&cue_storage, &entities).join() {
            if let Some(script) = script_storage.get_mut(cue.script.unwrap()) {
                script.cues.push(entity)
            }
        }
        info!("cues added to script components")
    }
}

pub struct ParseExpressions;
impl<'a> System<'a> for ParseExpressions {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::ParsedExp>,
        WriteStorage<'a, data_store::Span>,
        ReadStorage<'a, components::Variable>,
        ReadStorage<'a, components::Node>,
        Read<'a, ScriptProperties>,
    );

    fn run(
        &mut self,
        (entities, mut exp_storage, span_storage, var_storage, node_storage, scriptps): Self::SystemData,
    ) {
        for (var, var_entity, var_span) in (&var_storage, &entities, &span_storage).join() {
            let expression_chain = expression_parser::parse_expression(var.value.clone());
            let mut prior_type = vec![Datatypes::Component];
            for value in expression_chain.iter() {
                if let Some(node) = node_storage.get(var.node.unwrap()) {
                    if let Some((types, prior)) = expression_parser::infer_types(
                        var,
                        node,
                        value,
                        &prior_type,
                        scriptps.search(&value.text),
                    ) {
                        info!("{:?}", types);
                        info!("{:?}", prior);
                        prior_type = prior;
                        let parsed_exp_entity = entities.create();
                        let _ = exp_storage.insert(
                            parsed_exp_entity,
                            components::ParsedExp {
                                value: value.text.to_owned(),
                                script: var.script,
                                cue: var.cue,
                                node: var.node,
                                variable: Some(var_entity),
                                attr_name: var.name.to_owned(),
                                var_type: Some(value.exp.to_owned()),
                                possible_types: types,
                                // possible_types: vec![],
                                filepath: var.path.to_owned(),
                                span_start: var_span.start - 1 + value.start,
                                span_end: var_span.start - 1 + value.end,
                            },
                        );
                    }
                }
            }
        }
        info!("expression parsing complete");
        info!("expression_part_count: {:?}", exp_storage.count())
    }
}
