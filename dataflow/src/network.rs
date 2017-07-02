use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::BuildHasherDefault;

// Use 32-bit ints as indices to keep things compact.
type NodeId = u32;
type InputId = u32;

pub struct Network<N: fmt::Debug> {
    nodes: Vec<Option<Node<N>>>,
}

fn flatten<T>(o: Option<&Option<T>>) -> Option<&T> {
    match o {
        None | Some(&None) => None,
        Some(&Some(ref x)) => Some(x),
    }
}

fn flatten_mut<T>(o: Option<&mut Option<T>>) -> Option<&mut T> {
    match o {
        None | Some(&mut None) => None,
        Some(&mut Some(ref mut x)) => Some(x),
    }
}

impl<N: fmt::Debug> Network<N> {
    /// Create a new, empty network.
    pub fn new() -> Self {
        Network {
            nodes: Vec::new(),
        }
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

    /// Swap the input on a particular node.
    /// Ensure we disconnect from the current node and connect to the new one.
    pub fn set_input(
        &mut self, node_id: NodeId, input_id: InputId, target: NodeId) -> Result<(), NetworkError>
    {
        // TODO: check for cycle creation.
        let current_input_node_id = self.node_mut(node_id)?.input_node(input_id)?;
        // Make sure the target node exists before we alter anything.
        self.exists(target)?;

        // Unregister this node as a listener.
        match self.node_mut(current_input_node_id) {
            Ok(input_node) => input_node.remove_listener(node_id),
            Err(_) => error!(
                "Node {} had node {} as an input, but that node is not present in the network.",
                node_id,
                current_input_node_id),
        }
        // Register this node as a listener of the new node.
        self.node_mut(target)?.add_listener(node_id);
        
        // Set the value of the input to the new one.
        self.node_mut(target)?.set_input_node(input_id, target)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node<N: fmt::Debug> {
    /// Which other node Ids are listening to this one, and how many connections do they have?
    listeners: HashMap<NodeId, usize, BuildHasherDefault<simple_hash::SimpleHasher>>,
    inputs: Vec<NodeId>,
    inner: N,
}

impl<N: fmt::Debug> Node<N> {
    pub fn new(node: N) -> Self {
        Node {
            listeners: HashMap::<NodeId, usize, _>::with_hasher(
                BuildHasherDefault::<simple_hash::SimpleHasher>::default()),
            inputs: Vec::new(),
            inner: node,
        }
    }

    /// Get the node ID than an input is currently connected to.
    fn input_node(&self, id: InputId) -> Result<NodeId, NetworkError> {
        match self.inputs.get(id as usize) {
            Some(target) => Ok(*target),
            None => Err(noinput(id)),
        }
    }

    /// Set the target node for the provided input id.
    fn set_input_node(&mut self, id: InputId, target: NodeId) -> Result<(), NetworkError> {
        let node = self.inputs.get_mut(id as usize).ok_or(noinput(id))?;
        *node = target;
        Ok(())
    }

    /// Increment the listen count from another node.
    fn add_listener(&mut self, listener: NodeId) {
        *self.listeners.entry(listener).or_insert(0) += 1;
    }

    /// Decrement the listen count from another node.
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
    // This node ID isn't present in the network.
    NoNodeAt(NodeId),
    // This input ID is out of range for this node.
    InvalidInputId(InputId),
    // Connecting source to sink would create a cycle.
    WouldCycle{source: NodeId, sink: NodeId},
    // Internal error that indicates that we tried to access the same node mutably more than once.
    MultiMut(NodeId),
}

/// Shorthand for NetworkError::NoNodeAt.
fn nonode(id: NodeId) -> NetworkError {
    NetworkError::NoNodeAt(id)
}

/// Shorthand for NetworkError::InvalidInputId.
fn noinput(id: InputId) -> NetworkError {
    NetworkError::InvalidInputId(id)
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