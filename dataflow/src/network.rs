use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::{Entry, Keys};
use std::hash::BuildHasherDefault;
use std::mem::swap;

// Use 32-bit ints as indices to keep things compact.
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

    /// Remove a node from this network.  Fail if it has any listeners.
    pub fn remove(&mut self, node_id: NodeId) -> Result<N, NetworkError> {
        if self.node(node_id)?.has_listeners() {
            return Err(NetworkError::HasListeners(node_id));
        }
        // Move the node out of the collection.
        let mut node = None;
        swap(&mut node, &mut self.nodes[node_id as usize]);
        let node = node.unwrap(); // cannot fail as we just checked for node existence above.

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
        &mut self, node_id: NodeId, input_id: InputId, target: Option<NodeId>) -> Result<(), NetworkError>
    {
        // Make sure the target node exists before we alter anything.
        if let Some(t) = target {
            self.exists(t)?;
        }

        // FIXME: check for cycle creation.

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
}

#[derive(Debug, Copy, Clone)]
/// What are this node's requirements on its count of inputs?
pub enum InputCount {
    Fixed(u32),
    Variable,
    Range{min: u32, max: u32},
}

/// Trait expressing options that a node can express about its inputs, such as whether the number
/// should be fixed or variable.
pub trait Inputs {
    /// What count of allowed inputs does this node work with?
    fn allowed_count(&self) -> InputCount;
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

    /// Get an iterator over the node ids that are listening to this node.
    pub fn listeners(&self) -> Keys<NodeId, u32> {
        self.listeners.keys()
    }

    /// Get the node ID that an input is currently connected to.
    fn input_node(&self, id: InputId) -> Result<Option<NodeId>, NetworkError> {
        match self.inputs.get(id as usize) {
            Some(target) => Ok(*target),
            None => Err(noinput(id)),
        }
    }

    /// Set the target node for the provided input id.
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