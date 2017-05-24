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

use serde::{Serialize, Deserialize};
use rust_dmx::{DmxPort, Error as DmxPortError, OfflineDmxPort};

pub type DmxAddress = u16;
pub type DmxChannelCount = u16;
pub type DmxValue = u8;
pub type UniverseId = u32;
pub type FixtureId = u32;

const UNIVERSE_SIZE: usize = 512;

type UniverseSummary<T> = [Option<T>; UNIVERSE_SIZE];

pub struct Universe {
    name: String,
    port: Box<DmxPort>,
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

    pub fn write(&mut self) -> Result<(), DmxPortError> {
        self.port.write(&self.buffer)
    }
}

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
    pub fn add_universe(&mut self, universe: Universe) {
        let first_open_id = self.universes.iter().position(|u| u.is_none());
        match first_open_id {
            Some(id) => self.universes[id] = Some(universe),
            None => self.universes.push(Some(universe)),
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
    pub fn add<F: DmxFixture + 'static>(&mut self, fixture: F) {
        let id = self.next_id();
        let kind = fixture.kind();
        let item = PatchItem {
            id: id,
            name: kind.to_string(),
            address: None,
            fixture: Box::new(fixture),
        };
        self.items.push(item);
    }

    /// Remove a fixture by id, if it exists, and return it.
    pub fn remove(&mut self, id: FixtureId) -> Option<PatchItem> {
        match self.items.iter().position(|item| item.id == id) {
            Some(index) => Some(self.items.swap_remove(index)),
            None => None
        }
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
    pub fn patch(
            &mut self,
            id: FixtureId,
            universe: UniverseId,
            address: DmxAddress)
            -> Result<(), PatchError> {
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
            Err(PatchError::AddressConflict(conflicting_fixtures))
        }
    }

    /// Unpatch a fixture.
    pub fn unpatch(&mut self, id: FixtureId) -> Result<(), PatchError> {
        self.item_mut(id)?.address = None;
        Ok(())
    }

    /// Render every fixture to DMX.
    pub fn render(&mut self) -> Vec<DmxPortError> {
        for item in self.items.iter() {
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

pub struct PatchItem {
    id: FixtureId,
    pub name: String,
    address: Option<(UniverseId, DmxAddress)>,
    /// Trait object implementing the Fixture interface.
    fixture: Box<DmxFixture>,
}

impl PatchItem {
    /// Unique fixture id.
    fn id(&self) -> FixtureId { 
        self.id
    }
    /// The name of this kind of fixture.
    fn kind(&self) -> &'static str {
        self.fixture.kind()
    }

    fn universe(&self) -> Option<UniverseId> {
        self.address.map(|(id, _)| id)
    }

    fn channel_count(&self) -> DmxChannelCount {
        self.fixture.channel_count()
    }
}

pub trait DmxFixture {
    /// What kind of fixture is this?
    fn kind(&self) -> &'static str;

    /// Get the number of DMX channels that this fixture requires.
    fn channel_count(&self) -> u16;

    /// Render this fixture into a DMX buffer.
    /// The buffer provided will be of length channel_count.
    fn render(&self, buffer: &mut [DmxValue]);
}

pub enum PatchError {
    InvalidFixtureId(FixtureId),
    InvalidUniverseId(UniverseId),
    AddressConflict(Vec<FixtureId>),
    FixtureTooLongForAddress(DmxAddress, DmxChannelCount),
    NonEmptyUniverse(UniverseId),
    PortError(DmxPortError),
}
