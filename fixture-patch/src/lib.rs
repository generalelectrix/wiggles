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

use std::fmt;

use rust_dmx::{DmxPort, Error as DmxPortError, OfflineDmxPort};
use profiles::{Profile, PROFILES};
use fixture::{DmxFixture, DmxValue, DmxChannelCount};

mod fixture;
mod profiles;
mod test;

pub type DmxAddress = u16;
/// Return the address if it is valid, or an error.
fn valid_address(a: DmxAddress) -> Result<DmxAddress, PatchError> {
    if a < 511 {
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
    name: String,
    #[serde(with="rust_dmx")]
    port: Box<DmxPort>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[serde(default="empty_buffer")]
    buffer: [DmxValue; UNIVERSE_SIZE],
}

impl Universe {
    pub fn new(name: String, port: Box<DmxPort>) -> Self {
        Universe {
            name: name,
            port: port,
            buffer: [0; UNIVERSE_SIZE],
        }
    }

    pub fn new_offline<N: Into<String>>(name: N) -> Self {
        let port = Box::new(OfflineDmxPort{});
        Universe::new(name.into(), port)
    }

    pub fn set_port(&mut self, port: Box<DmxPort>) {
        self.port = port;
    }

    pub fn write(&mut self) -> Result<(), DmxPortError> {
        self.port.write(&self.buffer)
    }
}

impl fmt::Debug for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Universe {{ name: {}, port: {:?} }}", self.name, self.port.serializable())
    }
}

impl PartialEq for Universe {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        && self.port.serializable() == other.port.serializable()
        && self.buffer[..] == other.buffer[..]
    }
}

impl Eq for Universe {}

// -------------------------
// Single patched item
// -------------------------

#[derive(Serialize, Deserialize)]
pub struct PatchItem {
    id: FixtureId,
    pub name: String,
    address: Option<(UniverseId, DmxAddress)>,
    active: bool,
    fixture: DmxFixture,
}

impl PatchItem {
    /// Unique fixture id.
    fn id(&self) -> FixtureId { 
        self.id
    }
    /// The name of this kind of fixture.
    fn kind(&self) -> &str {
        self.fixture.kind()
    }

    fn universe(&self) -> Option<UniverseId> {
        self.address.map(|(id, _)| id)
    }

    fn channel_count(&self) -> DmxChannelCount {
        self.fixture.channel_count()
    }
}

// -------------------------
// The whole patch
// -------------------------

#[derive(Serialize, Deserialize)]
pub struct Patch {
    universes: Vec<Option<Universe>>,
    items: Vec<PatchItem>,
    next_id: FixtureId,
}

impl Patch {
    pub fn new() -> Self {
        Patch {
            universes: Vec::new(),
            items: Vec::new(),
            next_id: 0,
        }
    }

    /// Return a vec of tuples of (universe_id, name).
    pub fn describe_universes(&self) -> Vec<(UniverseId, &str)> {
        let mut descriptions = Vec::new();
        for (i, maybe_u) in self.universes.iter().enumerate() {
            if let &Some(ref u) = maybe_u {
                descriptions.push((i as UniverseId, u.name.as_str()));
            }
        }
        descriptions
    }

    /// Get an immutable reference to the current patches.
    pub fn items(&self) -> &Vec<PatchItem> {
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
    pub fn remove_universe(&mut self, id: UniverseId, force: bool) -> Result<(), PatchError> {

        if !force && self.items.iter().any(|item| item.universe() == Some(id)) {
            return Err(PatchError::NonEmptyUniverse(id))
        }

        /// unpatch any fixtures that are patched in this universe
        for item in self.items.iter_mut().filter(|item| item.universe() == Some(id)) {
            item.address = None;
        }

        *self.universes.get_mut(id as usize).ok_or(PatchError::InvalidUniverseId(id))? = None;
        Ok(())
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
        let item = PatchItem {
            id: id,
            name: name.unwrap_or(profile.name().to_string()),
            address: None,
            active: true,
            fixture: profile.create_fixture(),
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
                    self.remove(id);
                    Err(e)
                }
            }
        }

    /// Remove a fixture by id, if it exists, and return it.
    pub fn remove(&mut self, id: FixtureId) -> Option<PatchItem> {
        match self.items.iter().position(|item| item.id == id) {
            Some(index) => Some(self.items.swap_remove(index)),
            None => None
        }
    }

    /// Get an immutable reference to a patch item by id, if it exists.
    pub fn item(&self, id: FixtureId) -> Result<&PatchItem, PatchError> {
        self.items.iter().find(|item| item.id == id).ok_or(PatchError::InvalidFixtureId(id))
    }

    fn item_mut(&mut self, id: FixtureId) -> Result<&mut PatchItem, PatchError> {
        self.items.iter_mut().find(|item| item.id == id).ok_or(PatchError::InvalidFixtureId(id))
    }

    fn universe(&self, id: UniverseId) -> Result<&Universe, PatchError> {
        match self.universes.get(id as usize) {
            None | Some(&None) => Err(PatchError::InvalidUniverseId(id)),
            Some(&Some(ref u)) => Ok(u),
        }
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
            for addr_slot in &mut summary[addr..addr+item.channel_count() as usize] {
                *addr_slot = Some(id);
            }
        }

        Ok(summary)
    }

    /// Try to patch a fixture at a particular address in a particular universe.
    pub fn repatch(
            &mut self,
            id: FixtureId,
            universe: UniverseId,
            address: DmxAddress)
            -> Result<(), PatchError> {
        let address = valid_address(address)?;
        let univ_summary = self.universe_summary(universe)?;
        // get the relevant fixture
        let item = self.item_mut(id)?;

        let n_chan = item.channel_count();
        if (address + n_chan) as usize > UNIVERSE_SIZE {
            return Err(PatchError::FixtureTooLongForAddress(address, n_chan));
        }
        
        let proposed_channels = &univ_summary[address as usize..(address+n_chan) as usize];
        let mut conflicting_fixtures = proposed_channels.iter().filter_map(|c| *c).collect::<Vec<_>>();
        if conflicting_fixtures.is_empty() {
            // No conflicts, good to patch.
            item.address = Some((universe, address));
            Ok(())
        }
        else {
            // Deduplicate the list of conflicting fixture ids.
            conflicting_fixtures.dedup();
            Err(PatchError::AddressConflict(id, universe, address, conflicting_fixtures))
        }
    }

    /// Unpatch a fixture.
    pub fn unpatch(&mut self, id: FixtureId) -> Result<(), PatchError> {
        self.item_mut(id)?.address = None;
        Ok(())
    }

    /// Set the render state of a fixture.  If render is False, then the fixture will be ignored
    /// during DMX output.
    pub fn set_active(&mut self, id: FixtureId, state: bool) -> Result<(), PatchError> {
        self.item_mut(id)?.active = state;
        Ok(())
    }

    /// Render every fixture to DMX.
    pub fn render(&mut self) -> Vec<DmxPortError> {
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
                    let channel_count = item.channel_count();
                    let buf_slice = &mut univ.buffer[addr as usize..(addr+channel_count) as usize];
                    item.fixture.render(buf_slice);
                }
            }
        }

        /// Write every universe to its port, returning any errors to the caller.
        let mut write_errs = Vec::new();
        for maybe_u in self.universes.iter_mut() {
            match *maybe_u {
                Some(ref mut u) => {
                    if let Err(e) = u.write() {
                        write_errs.push(e);
                    }
                },
                None => {}
            }
        }
        write_errs
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
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            PatchError::PortError(ref pe) => Some(pe),
            _ => None,
        }
    }

}
