//! Abstract notion of a collection of "patched" fixtures.
//! The notion of patching should be more generic than just DMX, but provide support for some
//! extensible set of output connections a fixture can produce output on.  Ideally there should
//! be no restriction on how many different output connections a single fixture can have, such that
//! one logical fixture could span multiple control formats.
//! That said, for now we'll just support DMX for expediency.  Non-DMX control is still a pipe
//! dream anyway.
//! All DMX addresses are indexed from 0.  Conversion to index from 1 is left to the client.
extern crate rust_dmx;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate wiggles_value;
#[macro_use] extern crate lazy_static;
#[cfg(test)] extern crate serde_json;
#[cfg(test)] extern crate bincode;

use std::fmt;
use std::slice::Iter;
use wiggles_value::{Data, Datatype};
use rust_dmx::{DmxPort, Error as DmxPortError, OfflineDmxPort};
pub use profiles::{Profile, PROFILES};
pub use fixture::{DmxFixture, DmxValue, DmxChannelCount, FixtureControl};

mod fixture;
mod profiles;
mod test;

/// DmxAddress, indexed from 1!  When indexing into a buffer, make sure to subtract 1.
pub type DmxAddress = u16;
/// Return the address if it is valid, or an error.
fn valid_address(a: DmxAddress) -> Result<DmxAddress, PatchError> {
    if a > 0 && a < 513 {
        Ok(a)
    }
    else {
        Err(PatchError::InvalidDmxAddress(a))
    }
    
}
pub type UniverseId = u32;
pub type FixtureId = u32;

// -----------------------
// DMX Universe
// -----------------------

const UNIVERSE_SIZE: usize = 512;

type UniverseSummary<T> = [Option<T>; UNIVERSE_SIZE];

fn empty_buffer() -> [DmxValue; UNIVERSE_SIZE] {
    [0; UNIVERSE_SIZE]
}

#[derive(Serialize, Deserialize)]
pub struct Universe {
    #[serde(with="rust_dmx")]
    port: Box<DmxPort>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[serde(default="empty_buffer")]
    buffer: [DmxValue; UNIVERSE_SIZE],
}

impl fmt::Debug for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.port)
    }
}

impl Universe {
    pub fn new(port: Box<DmxPort>) -> Self {
        Universe {
            port: port,
            buffer: [0; UNIVERSE_SIZE],
        }
    }

    pub fn new_offline() -> Self {
        Universe::new(Box::new(OfflineDmxPort))
    }

    pub fn port_namespace(&self) -> &str {
        self.port.namespace()
    }

    pub fn port_id(&self) -> &str {
        self.port.port_name()
    }

    pub fn set_port(&mut self, port: Box<DmxPort>) {
        self.port = port;
    }

    pub fn write(&mut self) -> Result<(), DmxPortError> {
        self.port.write(&self.buffer)
    }
}

impl PartialEq for Universe {
    fn eq(&self, other: &Self) -> bool {
        self.port.serializable() == other.port.serializable()
        && self.buffer[..] == other.buffer[..]
    }
}

impl Eq for Universe {}

// -------------------------
// Single patched item
// -------------------------

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchItem<S> {
    id: FixtureId,
    pub name: String,
    address: Option<(UniverseId, DmxAddress)>,
    active: bool,
    fixture: DmxFixture,
    control_sources: Vec<Option<S>>,
}

impl<S> PatchItem<S> {
    /// Unique fixture id.
    pub fn id(&self) -> FixtureId { 
        self.id
    }
    /// The name of this kind of fixture.
    pub fn kind(&self) -> &str {
        self.fixture.kind()
    }

    /// The universe ID in which this fixture is patched, if any.
    pub fn universe(&self) -> Option<UniverseId> {
        self.address.map(|(id, _)| id)
    }

    /// The DMX address this fixture is patched at, if it is patched.
    pub fn address(&self) -> Option<DmxAddress> {
        self.address.map(|(_, addr)| addr)
    }

    /// The universe and DMX address this fixture is patched at, if it is patched.
    pub fn global_address(&self) -> Option<(UniverseId, DmxAddress)> {
        self.address
    }

    /// The DMX channel count that this fixture requires.
    pub fn channel_count(&self) -> DmxChannelCount {
        self.fixture.channel_count()
    }

    /// Get an immutable slice of this patch item's control sources.
    pub fn control_sources(&self) -> &[Option<S>] {
        &self.control_sources
    }

    /// Get an iterator over this patch item's controls.
    pub fn controls(&self) -> Iter<FixtureControl> {
        self.fixture.controls()
    }

    /// Set all of the control values of this fixture by providing a data source to retrieve its
    /// inputs from.  The data source should return a default value rather than an error if one of
    /// the source IDs is invalid.
    /// TODO: decide how we want to handle source deletion, it would be nice to react to the source
    /// going away by disconnecting the input and propagating that change out into the rest of the
    /// world.
    pub fn set_controls<F>(&mut self, data_source: F)
        where F: Fn(&S, Datatype) -> Data
    {
        for (source, control) in self.control_sources.iter().zip(self.fixture.controls_mut()) {
            
            let data = match *source {
                Some(ref s) => data_source(s, control.data_type()),
                None => Data::default_with_type_hint(Some(control.data_type())),
            };
            control.set_value(data);
        }
    }
}

// -------------------------
// The whole patch
// -------------------------

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patch<S> {
    universes: Vec<Option<Universe>>,
    items: Vec<PatchItem<S>>,
    next_id: FixtureId,
}

impl<S> Patch<S> {
    pub fn new() -> Self {
        Patch {
            universes: Vec::new(),
            items: Vec::new(),
            next_id: 0,
        }
    }

    /// Return a vector referencing populated universes along with their IDs.
    pub fn universes(&self) -> Vec<(UniverseId, &Universe)> {
        let mut universes = Vec::new();
        for (i, maybe_u) in self.universes.iter().enumerate() {
            if let &Some(ref univ) = maybe_u {
                universes.push((i as UniverseId, univ))
            }
        }
        universes
    }

    /// Get an immutable reference to the current patches.
    pub fn items(&self) -> &Vec<PatchItem<S>> {
        &self.items
    }

    /// Add a universe to the first available id.
    pub fn add_universe(&mut self, universe: Universe) -> UniverseId {
        let first_open_id = self.universes.iter().position(|u| u.is_none());
        match first_open_id {
            Some(id) => {
                self.universes[id] = Some(universe);
                id as u32
            },
            None => {
                self.universes.push(Some(universe));
                (self.universes.len()-1) as u32
            },
        }
    }

    /// Delete an existing universe.
    /// If force=false, fail if any fixtures are patched in this universe.
    /// If successful, returns a list of fixtures that were unpatched, if any.
    pub fn remove_universe(&mut self, id: UniverseId, force: bool) -> Result<Vec<FixtureId>, PatchError> {

        if !force && self.items.iter().any(|item| item.universe() == Some(id)) {
            return Err(PatchError::NonEmptyUniverse(id))
        }

        *self.universes.get_mut(id as usize).ok_or(PatchError::InvalidUniverseId(id))? = None;

        let mut unpatched_fixtures = Vec::new();
        /// unpatch any fixtures that are patched in this universe
        for item in self.items.iter_mut().filter(|item| item.universe() == Some(id)) {
            item.address = None;
            unpatched_fixtures.push(item.id);
        }

        Ok(unpatched_fixtures)
    }

    /// Generate the next fixture id.
    fn next_id(&mut self) -> FixtureId {
        let next = self.next_id;
        self.next_id += 1;
        next
    }

    /// Add a new fixture into the patch without specifying an address or universe.
    /// Provide a name for the fixture, or autogenerate one.
    /// Return the fixture id.
    pub fn add(&mut self, profile: &Profile, name: Option<String>) -> FixtureId {
        let id = self.next_id();
        let fixture = profile.create_fixture();
        let sources = {
            let control_count = fixture.control_count();
            let mut s = Vec::with_capacity(control_count);
            for _ in 0..control_count {
                s.push(None);
            }
            s
        };
        let item = PatchItem {
            id: id,
            name: name.unwrap_or(profile.name().to_string()),
            address: None,
            active: true,
            fixture: fixture,
            control_sources: sources,
        };
        self.items.push(item);
        id
    }
    
    /// Try to add a new fixture into the patch with a specified address.
    /// Provide a name for the fixture, or autogenerate one.
    pub fn add_at_address(
        &mut self,
        profile: &Profile,
        name: Option<String>,
        universe: UniverseId,
        address: DmxAddress)
        -> Result<FixtureId, PatchError> {
            let id = self.add(profile, name);
            match self.repatch(id, universe, address) {
                Ok(_) => Ok(id),
                Err(e) => {
                    self.remove(id).expect("We just added this fixture, it must be present.");
                    Err(e)
                }
            }
        }

    /// Remove a fixture by id, if it exists, and return it.
    pub fn remove(&mut self, id: FixtureId) -> Result<PatchItem<S>, PatchError> {
        match self.items.iter().position(|item| item.id == id) {
            Some(index) => Ok(self.items.swap_remove(index)),
            None => Err(PatchError::InvalidFixtureId(id)),
        }
    }

    /// Get an immutable reference to a patch item by id, if it exists.
    pub fn item(&self, id: FixtureId) -> Result<&PatchItem<S>, PatchError> {
        self.items.iter().find(|item| item.id == id).ok_or(PatchError::InvalidFixtureId(id))
    }

    /// Get a mutable reference to a patch item by id, if it exists.
    pub fn item_mut(&mut self, id: FixtureId) -> Result<&mut PatchItem<S>, PatchError> {
        self.items.iter_mut().find(|item| item.id == id).ok_or(PatchError::InvalidFixtureId(id))
    }

    /// Get an immutable reference to a universe by id, if it exists.
    fn universe(&self, id: UniverseId) -> Result<&Universe, PatchError> {
        match self.universes.get(id as usize) {
            None | Some(&None) => Err(PatchError::InvalidUniverseId(id)),
            Some(&Some(ref u)) => Ok(u),
        }
    }

    /// Get a mutable reference to a universe by id, if it exists.
    fn universe_mut(&mut self, id: UniverseId) -> Result<&mut Universe, PatchError> {
        match self.universes.get_mut(id as usize) {
            None | Some(&mut None) => Err(PatchError::InvalidUniverseId(id)),
            Some(&mut Some(ref mut u)) => Ok(u),
        }
    }

    pub fn set_universe_port(&mut self, id: UniverseId, port: Box<DmxPort>) -> Result<(), PatchError> {
        Ok(self.universe_mut(id)?.set_port(port))
    }

    /// Return a summary of the contents of a universe.
    pub fn universe_summary(
            &self,
            id: UniverseId)
            -> Result<UniverseSummary<FixtureId>, PatchError> {
        self.universe(id)?;

        let mut summary = [None; UNIVERSE_SIZE];
        let items_in_univ = self.items.iter()
            .filter_map(|item| {
                match item.address {
                    Some((univ, addr)) if univ == id => Some((item, addr as usize)),
                    _ => None
                }
            });

        for (item, addr) in items_in_univ {
            let id = item.id;
            // Don't forget to subtract 1 to index!
            let addr = addr - 1;
            for addr_slot in &mut summary[addr..addr+item.channel_count() as usize] {
                *addr_slot = Some(id);
            }
        }

        Ok(summary)
    }

    /// Try to patch a fixture at a particular address in a particular universe.
    /// If successful, return a reference to the repatched item.
    pub fn repatch(
            &mut self,
            id: FixtureId,
            universe: UniverseId,
            address: DmxAddress)
            -> Result<&PatchItem<S>, PatchError> {
        let address = valid_address(address)?;
        // Use this value for indexes!
        let address_from_zero = address - 1;
        let mut univ_summary = self.universe_summary(universe)?;
        // get the relevant fixture
        let item = self.item_mut(id)?;

        // If this fixture is in this universe, mark its current channels as unoccupied in the
        // universe summary to ensure we don't find self-conflicts.
        if Some(universe) == item.universe() {
            for chan in univ_summary.iter_mut() {
                if *chan == Some(id) {
                    *chan = None;
                }
            }
        }

        let n_chan = item.channel_count();
        if (address_from_zero + n_chan) as usize > UNIVERSE_SIZE {
            return Err(PatchError::FixtureTooLongForAddress(address, n_chan));
        }
        
        let proposed_channels = &univ_summary[address_from_zero as usize..(address_from_zero+n_chan) as usize];
        let mut conflicting_fixtures = proposed_channels.iter().filter_map(|c| *c).collect::<Vec<_>>();
        if conflicting_fixtures.is_empty() {
            // No conflicts, good to patch.
            item.address = Some((universe, address));
            Ok(item)
        }
        else {
            // Deduplicate the list of conflicting fixture ids.
            conflicting_fixtures.dedup();
            Err(PatchError::AddressConflict(id, universe, address, conflicting_fixtures))
        }
    }

    /// Unpatch a fixture.
    /// Return a reference to the item if it exists.
    pub fn unpatch(&mut self, id: FixtureId) -> Result<&PatchItem<S>, PatchError> {
        let item = self.item_mut(id)?;
        item.address = None;
        Ok(item)
    }

    /// Set the render state of a fixture.  If render is False, then the fixture will be ignored
    /// during DMX output.
    pub fn set_active(&mut self, id: FixtureId, state: bool) -> Result<(), PatchError> {
        self.item_mut(id)?.active = state;
        Ok(())
    }

    /// Set the control source for a particular control ID to the provided value.
    pub fn set_control_source(
        &mut self,
        id: FixtureId,
        control_id: usize,
        source: Option<S>)
        -> Result<(), PatchError>
    {
        let item = self.item_mut(id)?;
        match item.control_sources.get_mut(control_id) {
            Some(cs) => {
                *cs = source;
                Ok(())
            }
            None => Err(PatchError::ControlOutOfRange{
                fixture: id,
                control_id: control_id,
                control_count: item.fixture.control_count(),
            }),
        }
    }

    /// Set all of the control values of every fixture.
    pub fn set_controls<F>(&mut self, data_source: F)
        where F: Fn(&S, Datatype) -> Data
    {
        for ref mut item in &mut self.items {
            item.set_controls(&data_source);
        }
    }

    /// Render every fixture to DMX.
    pub fn render(&mut self) -> Vec<(UniverseId, DmxPortError)> {
        // Zero out every universe buffer.
        for univ_opt in self.universes.iter_mut() {
            match *univ_opt {
                Some(ref mut u) => {u.buffer = empty_buffer()},
                None => {},
            }
        }

        for item in self.items.iter() {
            if ! item.active {
                continue;
            }
            if let Some((univ_id, addr)) = item.address {
                if let Some(&mut Some(ref mut univ)) = self.universes.get_mut(univ_id as usize) {
                    let addr_from_zero = addr - 1;
                    let channel_count = item.channel_count();
                    let buf_slice = &mut univ.buffer[addr_from_zero as usize..(addr_from_zero+channel_count) as usize];
                    item.fixture.render(buf_slice);
                }
            }
        }

        /// Write every universe to its port, returning any errors to the caller.
        let mut write_errs = Vec::new();
        for (uid, maybe_u) in self.universes.iter_mut().enumerate() {
            match *maybe_u {
                Some(ref mut u) => {
                    if let Err(e) = u.write() {
                        write_errs.push((uid as UniverseId, e));
                    }
                },
                None => {}
            }
        }
        write_errs
    }
}

impl<S> Default for Patch<S> {
    fn default() -> Self {
        Patch::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatchError {
    InvalidDmxAddress(DmxAddress),
    InvalidFixtureId(FixtureId),
    InvalidUniverseId(UniverseId),
    AddressConflict(FixtureId, UniverseId, DmxAddress, Vec<FixtureId>),
    FixtureTooLongForAddress(DmxAddress, DmxChannelCount),
    NonEmptyUniverse(UniverseId),
    PortError(DmxPortError),
    ControlOutOfRange{fixture: FixtureId, control_id: usize, control_count: usize},
}

impl fmt::Display for PatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PatchError::*;
        match *self {
            InvalidDmxAddress(addr) => write!(f, "Invalid DMX address: {}", addr),
            InvalidFixtureId(id) => write!(f, "Invalid fixture id: {}.", id),
            InvalidUniverseId(id) => write!(f, "Invalid universe id: {}.", id),
            AddressConflict(fix, univ, addr, ref conflicts) => 
                write!(
                    f,
                    "Address conflict: fixture {}, universe {}, address {}. Conflicts with fixtures {:?}",
                    fix,
                    univ,
                    addr,
                    conflicts),
            FixtureTooLongForAddress(addr, count) =>
                write!(
                    f,
                    "Fixture of channel count {} is too long to be patched at address {}.",
                    count,
                    addr),
            NonEmptyUniverse(id) => write!(f, "Universe {} is not empty.", id),
            PortError(ref e) => write!(f, "DMX port error: {}", e),
            ControlOutOfRange{fixture, control_id, control_count} =>
                write!(
                    f,
                    "Fixture {} only has {} controls, {} is out of range.",
                    fixture,
                    control_id,
                    control_count,
                ),
        }
    }
}

impl std::error::Error for PatchError {
    fn description(&self) -> &str {
        use PatchError::*;
        match *self {
            InvalidDmxAddress(_) => "Invalid DMX address.",
            InvalidFixtureId(_) => "Invalid fixture id.",
            InvalidUniverseId(_) => "Invalid universe id.",
            AddressConflict(..) => "Addressing conflict.",
            FixtureTooLongForAddress(..) => "Channel count too high for address.",
            NonEmptyUniverse(_) => "Universe is not empty.",
            PortError(ref pe) => pe.description(),
            ControlOutOfRange{..} => "Control ID out of range.",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            PatchError::PortError(ref pe) => Some(pe),
            _ => None,
        }
    }

}
