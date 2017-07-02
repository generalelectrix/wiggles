use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::BuildHasherDefault;
use std::mem::swap;
use std::u32;

// Use 32-bit ints as indices to keep things compact.
// TODO: use a generation ID to ensure we don't reference an old version of a node.
type NodeId = u32;
type InputId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Network<N>
    where N: fmt::Debug + Inputs
{
    /// Collection of nodes, indexed by their ID.
    /// Indices are stable under insertion and deletion.
    nodes: Vec<Option<Node<N>>>,
}

impl<N> Network<N>
    where N: fmt::Debug + Inputs
{
    /// Create a new, empty network.
    pub fn new() -> Self {
        Network {
            nodes: Vec::new(),
        }
    }

    /// If there is an empty slot, return its id.  Otherwise return None.
    fn next_available_slot(&self) -> Option<usize> {
        for (i, slot) in self.nodes.iter().enumerate() {
            if slot.is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Insert a new node into this network.  Return a mutable reference to it.
    pub fn add(&mut self, node_contents: N) -> &mut Node<N> {
        let node = Node::new(node_contents);
        match self.next_available_slot() {
            Some(idx) => {
                insert_and_get_mut(&mut self.nodes, idx, node)
            }
            None => {
                push_and_get_mut(&mut self.nodes, node)
            }
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
        swap(&mut node, &mut self.nodes[node_id as usize]);
        let node = node.expect("Could not get a node whose existence we just verified.");

        // Safe to remove; iterate over its inputs and disconnect it, logging any errors.
        for input_node_id in node.inputs.iter().filter_map(|x| *x) {
            match self.node_mut(input_node_id) {
                Ok(input_node) => input_node.remove_listener(node_id),
                Err(_) => error!(
                    "Found an invalid input node {} while removing node {}.",
                    input_node_id,
                    node_id),
            }
        }
        // Now iterate over all of its listeners and disconnect them.
        for listener in node.listeners.keys() {
            match self.node_mut(*listener) {
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
        match flatten(self.nodes.get(id as usize)) {
            Some(_) => Ok(()),
            None => Err(nonode(id))
        }
    }

    /// Get an immutable reference to a node, if it exists.
    pub fn node(&self, id: NodeId) -> Result<&Node<N>, NetworkError> {
        let maybe_node = flatten(self.nodes.get(id as usize));
        maybe_node.ok_or(NetworkError::NoNodeAt(id))
    }

    /// Get a mutable reference to a node, if it exists.
    fn node_mut(&mut self, id: NodeId) -> Result<&mut Node<N>, NetworkError> {
        let maybe_node = flatten_mut(self.nodes.get_mut(id as usize));
        maybe_node.ok_or(NetworkError::NoNodeAt(id))
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
                Ok(input_node) => input_node.remove_listener(node_id),
                Err(_) => error!(
                    "Node {} had node {} as an input, but that node is not present in the network.",
                    node_id,
                    current_input_node_id),
            }
        }

        // Register this node as a listener of the new node, if we're not disconnecting it.
        if let Some(t) = target {
            self.node_mut(t)?.add_listener(node_id);
        }
        
        // Set the value of the input to the new one.
        self.node_mut(node_id)?.set_input_node(input_id, target)?;

        Ok(())
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
        // create a cycle.
        // Since non-cyclic is an invariant of this graph, we know that any cycle created by
        // connecting these nodes must involve both of them, so we don't need to do full DFS.
        if self.node_among_listeners(sink_id, source_id) {
            return Err(NetworkError::WouldCycle {source: source_id, sink: sink_id });
        }
        Ok(())
    }

    // TODO: determine if we want to keep track of visited nodes and skip them.
    /// Return True if this node ID is among this node's listeners, recursing down until we've
    /// plumbed the entire downstream graph.  Return early if we find the provided node.
    fn node_among_listeners(&self, node_to_check: NodeId, node_to_find: NodeId) -> bool {
        // log an error if we come across a node that should exist but doesn't.
        match self.node(node_to_check) {
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
    /// accomodate the new input.
    fn try_push_input(&mut self) -> Result<(), ()>;

    /// Tell this node we want to pop the last input.
    /// It should return Ok if this is allowed and must have done any work to ready itself for the
    /// change.
    fn try_pop_input(&mut self) -> Result<(), ()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node<N>
    where N: fmt::Debug + Inputs
{
    /// Which other node Ids are listening to this one, and how many connections do they have?
    listeners: HashMap<NodeId, u32, BuildHasherDefault<simple_hash::SimpleHasher>>,
    /// How many inputs does this node have, and what are they connected to (if anything).
    inputs: Vec<Option<NodeId>>,
    inner: N,
}

impl<N> Node<N>
    where N: fmt::Debug + Inputs
{
    pub fn new(node: N) -> Self {
        Node {
            listeners: HashMap::<NodeId, u32, _>::with_hasher(
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
    fn push_input(&mut self, target: Option<NodeId>) -> Result<InputId, ()> {
        // if we can't push another input, return an error
        // allow the caller to wrap it up into a real error value
        match self.inner.try_push_input() {
            Ok(()) => {
                // go ahead and add it
                self.inputs.push(target);
                Ok((self.inputs.len() - 1) as InputId)
            }
            Err(()) => Err(()),
        }
    }

    /// Pop the last input off this node, if it allows it.
    /// Return the target that was assigned to this input.
    /// The caller must ensure that this node has been removed as a listener of the node that was
    /// assigned to this input, if any.
    fn pop_input(&mut self) -> Result<Option<NodeId>, ()> {
        if self.inputs.is_empty() {
            return Err(());
        }
        match self.inner.try_pop_input() {
            Ok(()) => {
                // go ahead and remove
                let target = self.inputs.pop().expect("Pop on a list we know to not be empty failed.");
                Ok(target)
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
    fn add_listener(&mut self, listener: NodeId) {
        *self.listeners.entry(listener).or_insert(0) += 1;
    }

    /// Decrement the listener count from another node.
    /// Log an error if this node didn't have the other registered as a listener or if the
    /// listener count was 0.
    fn remove_listener(&mut self, listener: NodeId) {
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

pub enum NetworkError {
    /// This node ID isn't present in the network.
    NoNodeAt(NodeId),
    /// This input ID is out of range for this node.
    InvalidInputId(InputId),
    /// Connecting source to sink would create a cycle.
    WouldCycle{source: NodeId, sink: NodeId},
    /// Internal error that indicates that we tried to access the same node mutably more than once.
    MultiMut(NodeId),
    /// Cannot remove a node because it has listeners.
    HasListeners(NodeId),
    /// Cannot add an input to this node.
    CantAddInput(NodeId),
    /// Cannot remove an input from this node.
    CantRemoveInput(NodeId),
}

/// Shorthand for NetworkError::NoNodeAt.
fn nonode(id: NodeId) -> NetworkError {
    NetworkError::NoNodeAt(id)
}

/// Shorthand for NetworkError::InvalidInputId.
fn noinput(id: InputId) -> NetworkError {
    NetworkError::InvalidInputId(id)
}

/// Helper function to push an item onto a vec of options and immediately return a mutable reference
/// to the inner value.
fn push_and_get_mut<T>(v: &mut Vec<Option<T>>, item: T) -> &mut T {
    v.push(Some(item));
    v.last_mut().unwrap().as_mut().unwrap()
}

/// Helper function to insert an item into a vec of options and immediately return a mutable
/// reference to the inner value.  Panics if the index is out of range.
fn insert_and_get_mut<T>(v: &mut Vec<Option<T>>, index: usize, item: T) -> &mut T {
    let slot = &mut v[index];
    *slot = Some(item);
    slot.as_mut().unwrap()
}

/// Helper function to flatten a nested immutable option.
fn flatten<T>(o: Option<&Option<T>>) -> Option<&T> {
    match o {
        None | Some(&None) => None,
        Some(&Some(ref x)) => Some(x),
    }
}

/// Helper function to flatten a nested mutable option.
fn flatten_mut<T>(o: Option<&mut Option<T>>) -> Option<&mut T> {
    match o {
        None | Some(&mut None) => None,
        Some(&mut Some(ref mut x)) => Some(x),
    }
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