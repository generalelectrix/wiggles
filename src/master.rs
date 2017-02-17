//! Top-level entity that owns all data and routes events.
use std::collections::HashMap;
use clock_network::{
    ClockValue,
    ClockNetwork,
    ClockNodeIndex,
    ClockNodePrototype,
    ClockError,
    ClockEvent
};
use event::{Event, Events};
use knob::{KnobEvent, PatchBay};
use datatypes::{ErrorMessage, DeltaT};

pub struct Master {
    patch_bay: PatchBay,
    clock_network: ClockNetwork,
    clock_types: HashMap<String,ClockNodePrototype>,
}

type ApiResult = Result<Events,ErrorMessage>;

#[derive(Debug)]
pub struct RenderResponse {
    clock_values: Vec<Result<ClockValue,ErrorMessage>>
}

// These methods represent the public API for this package.
impl Master {
    pub fn update(&mut self, dt: DeltaT) -> Events {
        self.clock_network.update(dt)
    }

    pub fn render(&self, clock_nodes: &[ClockNodeIndex]) -> RenderResponse {
        let clock_values: Vec<_> = clock_nodes.iter()
            .map(|idx| 
                self.clock_network
                    .get_value_from_node(*idx)
                    .map_err(|r| r.into()))
            .collect();
        RenderResponse { clock_values: clock_values }
    }

    pub fn new_clock(
            &mut self,
            prototype_name: &str,
            name: String,
            inputs: &[ClockNodeIndex])
            -> ApiResult {
        // get a valid prototype or fail
        let proto =
            self.clock_types
                .get(prototype_name)
                .ok_or(ClockError::UnknownPrototype(prototype_name.to_string()))?;
        // create the new node
        let node = self.clock_network.add_node(proto, name.clone(), inputs)?;
        let mut events = Events::single(ClockEvent::NodeAdded{node: node.index(), name: name});

        // add knob patches for the new node
        // since we already added the node to the network, this operation must not fail or we
        // have a data integrity issue.
        events.push(self.patch_bay.add_clock_node(node));

        Ok(events)
    }
}