//! Top-level entity that owns all data and routes events.
use std::collections::HashMap;
use network::{NetworkNode, InputId};
use clock_network::{
    ClockValue,
    ClockNetwork,
    ClockNodeIndex,
    ClockNode,
    ClockError,
    ClockEvent,
};
use clocks::create_prototypes;
use data_network::{
    DataNetwork,
    DataNodeIndex,
    DataflowError,
    DataflowEvent,
};
use datatypes::Update;
use event::Events;
use knob::{KnobEvent, PatchBay, KnobPatch, KnobValue};
use datatypes::{ErrorMessage, DeltaT};

macro_rules! events {
    ( $( $event:expr ),* ) => (
        {
            let mut events = Events::new();
            $(
                events.push($event);
            )*
            Ok(events)
        }
    )
}


pub struct Master {
    patch_bay: PatchBay,
    clock_network: ClockNetwork,
    data_network: DataNetwork,
}

type ApiResult = Result<Events,ErrorMessage>;

#[derive(Debug)]
pub struct RenderResponse {
    clock_values: Vec<Result<ClockValue,ErrorMessage>>,
}

// These methods represent the public API for the interlocking dataflow networks and knob patch
// system.
impl Master {
    pub fn new() -> Self {
        Master {
            patch_bay: PatchBay::new(),
            clock_network: ClockNetwork::new(),
            data_network: DataNetwork::new(),
         }
    }

    pub fn update(&mut self, dt: DeltaT) -> Events {
        let mut clock_events = self.clock_network.update(dt);
        let data_events = self.data_network.update(dt);
        clock_events.extend(data_events);
        clock_events
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

    // methods relating to adding, removing, or modifying nodes from either network

    pub fn new_clock(&mut self, node: ClockNode) -> ApiResult {
        let node = self.clock_network.add_node(node)?;
        let ce = ClockEvent::NodeAdded{node: node.id(), name: node.name.clone()};

        // Add knob patches for the new node.
        // Since we already added the node to the network, this operation must not fail or we
        // have a data integrity issue.
        let ke = self.patch_bay.add_clock_node(node);
        events!(ce, ke)
    }

    pub fn delete_clock(&mut self, node: ClockNodeIndex) -> ApiResult {
        let removed_node = self.clock_network.remove_node(node)?;
        // remove all related patches
        let ke = self.patch_bay.remove_clock_node(&removed_node);
        // signal the now-removed node
        let ce = ClockEvent::NodeRemoved{node: node, name: removed_node.name};
        events!(ke, ce)
    }

    pub fn swap_clock(&mut self, node_id: ClockNodeIndex, node: ClockNode) -> ApiResult {
        let new_node_name = node.name.clone();
        let old_node = self.clock_network.swap_node(node_id, node)?;

        let node_added = ClockEvent::NodeAdded{node: node_id, name: new_node_name};
        let node_removed = ClockEvent::NodeRemoved{node: node_id, name: old_node.name.clone()};

        // Remove the knob patches for the departing node.
        let ke_removed = self.patch_bay.remove_clock_node(&old_node);
        // Add knob patches for the incoming node.
        let ke_added = self.patch_bay.add_clock_node(self.clock_network.get_node(node_id)?);
        events!(node_removed, node_added, ke_removed, ke_added)
    }

    pub fn delete_data_node(&mut self, node: DataNodeIndex) -> ApiResult {
        let removed_node = self.data_network.remove_node(node)?;
        // remove all related patches
        let ke = self.patch_bay.remove_data_node(&removed_node);
        // unregister this node from the clock network
        for socket in removed_node.clock_input_sockets() {
            self.clock_network.remove_listener(socket.input, removed_node.id());
        }
        // signal the now-removed node
        let ce = DataflowEvent::NodeRemoved{node: node, name: removed_node.name};
        events!(ke, ce)
    }

    pub fn rename_clock(&mut self, node: ClockNodeIndex, name: String) -> ApiResult {
        self.clock_network.get_node_mut(node)?.name = name.clone();
        events!(ClockEvent::NodeRenamed {node: node, name: name})
    }

    pub fn rename_data_node(&mut self, node: DataNodeIndex, name: String) -> ApiResult {
        self.data_network.get_node_mut(node)?.name = name.clone();
        events!(DataflowEvent::NodeRenamed {node: node, name: name})
    }

    pub fn swap_clock_input(
        &mut self,
        node: ClockNodeIndex,
        input: InputId,
        new_source: ClockNodeIndex)
        -> ApiResult {
            events!(self.clock_network.swap_input(node, input, new_source)?)
    }

    pub fn swap_data_input(
        &mut self,
        node: DataNodeIndex,
        input: InputId,
        new_source: DataNodeIndex)
        -> ApiResult {
            events!(self.data_network.swap_input(node, input, new_source)?)
    }

    pub fn swap_data_clock_input(
        &mut self,
        node: DataNodeIndex,
        input: InputId,
        new_source: ClockNodeIndex)
        -> ApiResult {
            events!(self.data_network.swap_clock_input(node, input, new_source, &mut self.clock_network)?)
    }

    pub fn set_knob(&mut self, patch: KnobPatch, value: KnobValue) -> ApiResult {
        let e = self.patch_bay.set_knob_value(
            patch, value, &mut self.clock_network, &mut self.data_network)?;
        events!(e)
    }
}