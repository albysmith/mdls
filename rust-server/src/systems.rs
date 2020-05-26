use specs::prelude::*;
use crate::*;
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