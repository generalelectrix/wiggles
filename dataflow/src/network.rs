use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::BuildHasherDefault;
use std::mem::swap;
use std::u32;

// Use 32-bit ints as indices.
// Use a 32-bit generation ID to uniquely identify a generation of a particular slot to ensure that
// we don't try to mess with an old version of a node if it has been removed and re-used later.
type NodeIndex = u32;
type GenerationId = u32;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeId(NodeIndex, GenerationId);

impl NodeId {
    pub fn index(&self) -> NodeIndex {
        self.0
    }

    pub fn gen_id(&self) -> GenerationId {
        self.1
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "node {}, generation {}", self.0, self.1)
    }
}

type InputId = u32;

#[derive(Debug, Serialize, Deserialize)]
/// Generation ID and a slot to hold onto a node in the network.
struct NodeSlot<N>
    where N: fmt::Debug + Inputs
{
    gen_id: GenerationId,
    node: Option<Node<N>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network<N>
    where N: fmt::Debug + Inputs
{
    /// Collection of node slots, indexed by their ID.
    /// Tagged internally with a generation ID.
    /// Indices are stable under insertion and deletion.
    /// Generation IDs are incremented when an empty slot is filled.
    slots: Vec<NodeSlot<N>>,
}

impl<N> Network<N>
    where N: fmt::Debug + Inputs
{
    /// Create a new, empty network.
    pub fn new() -> Self {
        Network {
            slots: Vec::new(),
        }
    }

    /// Insert a new node into this network.  Return a mutable reference to it.
    pub fn add(&mut self, node_contents: N) -> &mut Node<N> {
        let node = Node::new(node_contents);
        // Find the first available slot index, if one exists.
        // Sadly we can't do this more directly because of rustlang #21906.
        // If there aren't any, push the new node onto the end.
        let mut slot_idx = None;
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.node.is_none() {
                slot_idx = Some(index);
                break;
            }
        }

        if let Some(index) = slot_idx {
            let slot = self.slots.get_mut(index).expect("We just got this index, it must be live.");
            // increment the generation ID
            slot.gen_id += 1;
            // slip the node in
            slot.node = Some(node);
            // return a reference to it
            slot.node.as_mut().unwrap()
        }
        else {
            // no available slot, push a new slot on
            let slot = NodeSlot {gen_id: 0, node: Some(node)};
            self.slots.push(slot);
            // return a reference to the node we just added
            self.slots.last_mut().unwrap().node.as_mut().unwrap()
        }
    }

    /// Remove a node from this network.  Fail if it has any listeners unless we are forcing removal.
    pub fn remove(&mut self, node_id: NodeId, force: bool) -> Result<N, NetworkError> {
        // make sure this node exists
        self.exists(node_id)?;

        // fail if there are listeners unless we're forcing removal
        if !force && self.node(node_id)?.has_listeners() {
            return Err(NetworkError::HasListeners(node_id));
        }

        // Move the node out of the collection.
        let mut node = None;
        swap(&mut node, &mut self.slots[node_id.index() as usize].node);
        let node = node.expect("Could not get a node whose existence we just verified.");

        // Safe to remove; iterate over its inputs and disconnect it, logging any errors.
        for input_node_id in node.inputs.iter().filter_map(|x| *x) {
            match self.node_mut(input_node_id) {
                Ok(input_node) => input_node.remove_listener(node_id.index()),
                Err(_) => error!(
                    "Found an invalid input node {} while removing node {}.",
                    input_node_id,
                    node_id),
            }
        }
        // Now iterate over all of its listeners and disconnect them.
        for listener in node.listeners.keys() {
            match self.node_direct_mut(*listener) {
                Ok(node) => {
                    node.disconnect_from(node_id);
                }
                Err(_) => {
                    // log an error if one of this node's listeners didn't exist
                    error!(
                        "The node {} was registered as a listener of node {} (undergoing removal) \
                        but it is missing from the node collection.",
                        listener,
                        node_id,
                    );
                }
            }
        }
        Ok(node.inner)
    }

    /// Return Ok if this node id corresponds to a node in this network.
    fn exists(&self, id: NodeId) -> Result<(), NetworkError> {
        self.node(id).map(|_| ())
    }

    /// Get an immutable reference to a node, if it exists.
    pub fn node(&self, id: NodeId) -> Result<&Node<N>, NetworkError> {
        match self.slots.get(id.index() as usize) {
            None => Err(nonode(id)),
            Some(slot) => {
                match slot.node {
                    None => Err(nonode(id)),
                    Some(_) if slot.gen_id != id.gen_id() => Err(oldgen(id)),
                    Some(ref node) => Ok(node),
                }
            }
        }
    }

    /// Get an immutable reference to a node without specifying a generation ID, if it exists.
    fn node_direct(&self, index: NodeIndex) -> Result<&Node<N>, NetworkError> {
        match self.slots.get(index as usize) {
            None => Err(nonode(NodeId(index, 0))),
            Some(slot) => {
                match slot.node {
                    None => Err(nonode(NodeId(index, 0))),
                    Some(ref node) => Ok(node),
                }
            }
        }
    }

    /// Get a mutable reference to a node, if it exists.
    fn node_mut(&mut self, id: NodeId) -> Result<&mut Node<N>, NetworkError> {
        match self.slots.get_mut(id.index() as usize) {
            None => Err(nonode(id)),
            Some(slot) => {
                match slot.node {
                    None => Err(nonode(id)),
                    Some(_) if slot.gen_id != id.gen_id() => Err(oldgen(id)),
                    Some(ref mut node) => Ok(node),
                }
            }
        }
    }

    /// Get a mutable reference to a node without specifying a generation ID, if it exists.
    fn node_direct_mut(&mut self, index: NodeIndex) -> Result<&mut Node<N>, NetworkError> {
        match self.slots.get_mut(index as usize) {
            None => Err(nonode(NodeId(index, 0))),
            Some(slot) => {
                match slot.node {
                    None => Err(nonode(NodeId(index, 0))),
                    Some(ref mut node) => Ok(node),
                }
            }
        }
    }

    /// Swap an input on a particular node.
    /// Ensure we disconnect from the current node (if not None) and connect to the new one
    /// (if not None).
    pub fn swap_input(
        &mut self,
        node_id: NodeId,
        input_id: InputId,
        target: Option<NodeId>)
        -> Result<(), NetworkError>
    {
        // Validate that connecting these nodes wouldn't create a cycle.
        if let Some(t) = target {
            self.check_would_cycle(t, node_id)?;
        }

        if let Some(current_input_node_id) = self.node_mut(node_id)?.input_node(input_id)? {
            // Unregister this node as a listener if this input was already connected.
            match self.node_mut(current_input_node_id) {
                Ok(input_node) => input_node.remove_listener(node_id.index()),
                Err(_) => error!(
                    "Node {} had node {} as an input, but that node is not present in the network.",
                    node_id,
                    current_input_node_id),
            }
        }

        // Register this node as a listener of the new node, if we're not disconnecting it.
        if let Some(t) = target {
            self.node_mut(t)?.add_listener(node_id.index());
        }
        
        // Set the value of the input to the new one.
        self.node_mut(node_id)?.set_input_node(input_id, target)?;

        Ok(())
    }

    /// Push a new input onto a node, if it supports this operation.
    /// Return the new input ID and a potential message type returned by the node.
    pub fn push_input<M>(
            &mut self,
            node_id: NodeId,
            target: Option<NodeId>)
            -> Result<(InputId, M), NetworkError>
    {
        // if a target was supplied, make sure it exists
        if let Some(t) = target {
            self.exists(t)?;
        }
        match self.node_mut(node_id)?.push_input(target) {
            Ok(x) => {
                // if we had a target, add this node as a listener of it
                if let Some(t) = target {
                    // We know this node exists so this should never fail.
                    let target_node = self.node_mut(t).expect("We just checked that this node exists.");
                    target_node.add_listener(node_id.index());
                }
                Ok(x)
            }
            Err(()) => Err(NetworkError::CantAddInput(node_id)),
        }
    }

    /// Pop the last input off of a node, if it supports this operation.
    /// Return the message type returned by the node.
    pub fn pop_input<M>(
            &mut self,
            node_id: NodeId)
            -> Result<M, NetworkError>
    {
        match self.node_mut(node_id)?.pop_input() {
            Ok((target, msg)) => {
                // if this input had a target, remove this node as a listener.
                if let Some(t) = target {
                    // If this node doesn't exist, log an error.
                    match self.node_mut(t) {
                        Ok(target_node) => {
                            target_node.remove_listener(node_id.index());
                        }
                        Err(e) => {
                            error!(
                                "Popped an input off of node {}, but its target (node {}) could \
                                not be retrieved due to an error: {}.",
                                node_id.index(),
                                t,
                                e,
                            );
                        }
                    }
                    
                }
                Ok(msg)
            }
            Err(()) => Err(NetworkError::CantRemoveInput(node_id)),
        }
    }

    /// Return an appropriate error if connecting source to sink would create a cycle.
    /// Also serves to validate the existence of both node IDs.
    fn check_would_cycle(&self, source_id: NodeId, sink_id: NodeId) -> Result<(), NetworkError> {
        // first check some easy edge cases
        // if source and sink are the same, this would obviously cycle
        if source_id == sink_id {
            return Err(NetworkError::WouldCycle {source: source_id, sink: sink_id });
        }
        // if sink has no listeners, impossible to cycle
        // if source has no inputs, impossible to cycle
        let sink = self.node(sink_id)?;
        let source = self.node(source_id)?;
        if ! sink.has_listeners() {
            return Ok(());
        }
        if source.inputs.iter().all(|input| input.is_none()) {
            return Ok(());
        }

        // Checked the easy cases, now search for a cycle.
        // Dumb algorithm: starting at sink, iterate through all listeners, then those listeners'
        // listeners, until we hit bottom.  If we come across source_id among any of them, we would
        // create a cycle.  Since we've already checked that we have the right generation IDs, just
        // use node indexes for the search.
        // Since non-cyclic is an invariant of this graph, we know that any cycle created by
        // connecting these nodes must involve both of them, so we don't need to do full DFS.
        if self.node_among_listeners(sink_id.index(), source_id.index()) {
            return Err(NetworkError::WouldCycle {source: source_id, sink: sink_id });
        }
        Ok(())
    }

    // TODO: determine if we want to keep track of visited nodes and skip them.
    /// Return True if this node index is among this node's listeners, recursing down until we've
    /// plumbed the entire downstream graph.  Return early if we find the provided node.
    fn node_among_listeners(&self, node_to_check: NodeIndex, node_to_find: NodeIndex) -> bool {
        // log an error if we come across a node that should exist but doesn't.
        match self.node_direct(node_to_check) {
            Err(_) => {
                error!(
                    "Node {} should exist but was not found in the graph during cycle check.",
                    node_to_check,
                );
                false
            }
            Ok(node) => {
                // first just iterate over all of the listeners and check if any of them are the
                // node we're checking for (faster to check them all before recursing).
                for listener in node.listeners.keys() {
                    if *listener == node_to_find {
                        return true;
                    }
                }
                // we didn't find it, so now recurse into these listeners
                for listener in node.listeners.keys() {
                    if self.node_among_listeners(*listener, node_to_check) {
                        return true;
                    }
                }
                // we didn't find it among any listeners
                false
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// What are this node's requirements on its count of inputs?
pub struct InputCount {
    min: u32,
    max: u32,
}

impl InputCount {
    pub fn new(min: u32, max: u32) -> Self {
        InputCount {
            min: min,
            max: max,
        }
    }
    /// No inputs allowed.
    pub fn none() -> Self {
        InputCount::fixed(0)
    }

    /// A fixed number of inputs.
    pub fn fixed(n: u32) -> Self {
        InputCount::new(n, n)
    }

    /// 0 to N.
    pub fn up_to(n: u32) -> Self {
        InputCount::new(0, n)
    }

    /// At least this many.
    pub fn at_least(n: u32) -> Self {
        InputCount::new(n, u32::MAX)
    }

    /// Any number of inputs.
    pub fn any() -> Self {
        InputCount::new(0, u32::MAX)
    }

    /// Return true if this input count specifies that it is safe to push another input.
    pub fn can_push(&self, current_input_count: u32) -> bool {
        current_input_count < self.max
    }

    /// Return true if this input count specifies that it is safe to pop the last input.
    pub fn can_pop(&self, current_input_count: u32) -> bool {
        current_input_count > self.min
    }
}

/// Trait expressing options that a node can express about its inputs.
pub trait Inputs {
    /// How many inputs this node should default to when initialized.
    fn default_input_count() -> u32;
    /// Tell this node we're pushing another input.
    /// It should return Ok if this is allowed and must have done any work it needs to do to
    /// accomodate the new input.  The node is free to return an arbitrary data structure to the
    /// caller, which should probably hand it off to someone else for processing.
    fn try_push_input<M>(&mut self) -> Result<M, ()>;

    /// Tell this node we want to pop the last input.
    /// It should return Ok if this is allowed and must have done any work to ready itself for the
    /// change.
    fn try_pop_input<M>(&mut self) -> Result<M, ()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node<N>
    where N: fmt::Debug + Inputs
{
    /// Which other node Ids are listening to this one, and how many connections do they have?
    /// We just track node indices here rather than full node IDs with the generation ID as
    /// we should internally hold the invariant that removal of a node always ensures that we
    /// remove all listeners.
    listeners: HashMap<NodeIndex, u32, BuildHasherDefault<simple_hash::SimpleHasher>>,
    /// How many inputs does this node have, and what are they connected to (if anything).
    inputs: Vec<Option<NodeId>>,
    inner: N,
}

impl<N> Node<N>
    where N: fmt::Debug + Inputs
{
    pub fn new(node: N) -> Self {
        Node {
            listeners: HashMap::<NodeIndex, u32, _>::with_hasher(
                BuildHasherDefault::<simple_hash::SimpleHasher>::default()),
            inputs: Vec::new(),
            inner: node,
        }
    }

    /// Return True if this node's listener collection is not empty.
    /// This method assumes that we have ensured that the listeners collection has an entry
    /// removed immediately any time the listener count hits 0.
    pub fn has_listeners(&self) -> bool {
        self.listeners.is_empty()
    }

    /// Push another input onto this node, if it can support it.
    /// The caller must ensure this node has been added a listener of the target node if it is not
    /// None.
    /// Return the data structure returned by the node, probably a message type to be passed
    /// upstack.
    fn push_input<M>(&mut self, target: Option<NodeId>) -> Result<(InputId, M), ()> {
        // if we can't push another input, return an error
        // allow the caller to wrap it up into a real error value
        match self.inner.try_push_input() {
            Ok(msg) => {
                // go ahead and add it
                self.inputs.push(target);
                Ok((((self.inputs.len() - 1) as InputId), msg))
            }
            Err(()) => Err(()),
        }
    }

    /// Pop the last input off this node, if it allows it.
    /// Return the target that was assigned to this input.
    /// The caller must ensure that this node has been removed as a listener of the node that was
    /// assigned to this input, if any.
    fn pop_input<M>(&mut self) -> Result<(Option<NodeId>, M), ()> {
        if self.inputs.is_empty() {
            return Err(());
        }
        match self.inner.try_pop_input() {
            Ok(msg) => {
                // go ahead and remove
                let target = self.inputs.pop().expect("Pop on a list we know to not be empty failed.");
                Ok((target, msg))
            }
            Err(()) => Err(()),
        }
    }

    /// Get the node ID that an input is currently connected to.
    fn input_node(&self, id: InputId) -> Result<Option<NodeId>, NetworkError> {
        match self.inputs.get(id as usize) {
            Some(target) => Ok(*target),
            None => Err(noinput(id)),
        }
    }

    /// Set the target node for the provided input id.
    /// The caller must ensure correct update of relevant listeners.
    fn set_input_node(&mut self, id: InputId, target: Option<NodeId>) -> Result<(), NetworkError> {
        let node = self.inputs.get_mut(id as usize).ok_or(noinput(id))?;
        *node = target;
        Ok(())
    }

    /// Increment the listen count from another node.
    fn add_listener(&mut self, listener: NodeIndex) {
        *self.listeners.entry(listener).or_insert(0) += 1;
    }

    /// Decrement the listener count from another node.
    /// Log an error if this node didn't have the other registered as a listener or if the
    /// listener count was 0.
    fn remove_listener(&mut self, listener: NodeIndex) {
        let mut should_remove = false;
        // If the listener was missing completely, Some(false).
        // If the listener was present but the count was zero, Some(true).
        let mut error = None;
        match self.listeners.entry(listener) {
            Entry::Occupied(ref mut count) => {
                if *count.get() == 0 {
                    error = Some(true);
                    should_remove = true;
                }
                else {
                    *count.get_mut() -= 1;
                    if *count.get() == 0 {
                        should_remove = true;
                    }
                }
            }
            Entry::Vacant(_) => {
                error = Some(false);
            }
        }
        if should_remove {
            self.listeners.remove(&listener);
        }
        if let Some(present) = error {
            let err_reason =
                if present {
                    "the listener count is already 0"
                }
                else {
                    "the listener was not registered"
                };
            error!(
                "Tried to remove the listener {} from node {:?} but {}.",
                listener,
                self,
                err_reason,
            );
        }
    }

    /// Disconnect this node's inputs from the provided listener.
    fn disconnect_from(&mut self, target: NodeId) {
        for input in self.inputs.iter_mut() {
            if *input == Some(target) {
                *input = None;
            }
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkError {
    /// A command was directed at a node using an outdated generation ID.
    OldGenId(NodeId),
    /// This node ID's index isn't present in the network.
    NoNodeAt(NodeId),
    /// This input ID is out of range for this node.
    InvalidInputId(InputId),
    /// Connecting source to sink would create a cycle.
    WouldCycle{source: NodeId, sink: NodeId},
    /// Cannot remove a node because it has listeners.
    HasListeners(NodeId),
    /// Cannot add an input to this node.
    CantAddInput(NodeId),
    /// Cannot remove an input from this node.
    CantRemoveInput(NodeId),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::NetworkError::*;
        match *self {
            OldGenId(id) =>
                write!(f, "Outdated generation id ({}) for node {}.", id.gen_id(), id.index()),
            NoNodeAt(id) =>
                write!(f, "No node at index {}.", id.index()),
            InvalidInputId(id) =>
                write!(f, "Input id {} is out of range.", id),
            WouldCycle{source, sink} =>
                write!(
                    f,
                    "Connecting source {} to sink {} would create a cycle.",
                    source.index(),
                    sink.index()),
            HasListeners(id) =>
                write!(f, "Node {} has listeners.", id.index()),
            CantAddInput(id) =>
                write!(f, "Cannot add an input to node {}.", id.index()),
            CantRemoveInput(id) =>
                write!(f, "Cannot remove an input from node {}.", id.index()),
        }
    }
}

/// Shorthand for NetworkError::NoNodeAt.
fn nonode(id: NodeId) -> NetworkError {
    NetworkError::NoNodeAt(id)
}

/// Shorthand for NetworkError::InvalidInputId.
fn noinput(id: InputId) -> NetworkError {
    NetworkError::InvalidInputId(id)
}

/// Shorthand constructor for NetworkError::OldGenId.
fn oldgen(id: NodeId) -> NetworkError {
    NetworkError::OldGenId(id)
}

/// Implement dirt-simple hash function that just uses the integer you've given it as the hash key.
/// Taken from https://gist.github.com/arthurprs/88eef0b57b9f8341c54e2d82ec775698
mod simple_hash {
    use std::hash::Hasher;
    pub struct SimpleHasher(u64);

    #[inline]
    fn load_u64_le(buf: &[u8], len: usize) -> u64 {
        use std::ptr;
        debug_assert!(len <= buf.len());
        let mut data = 0u64;
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), &mut data as *mut _ as *mut u8, len);
        }
        data.to_le()
    }


    impl Default for SimpleHasher {

        #[inline]
        fn default() -> SimpleHasher {
            SimpleHasher(0)
        }
    }

    impl Hasher for SimpleHasher {

        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            *self = SimpleHasher(load_u64_le(bytes, bytes.len()));
        }
    }
}