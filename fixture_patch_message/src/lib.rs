//! Message passing based API for the fixture patch.
extern crate wiggles_value;
extern crate fixture_patch;
extern crate console_server;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate rust_dmx;

use std::fmt;
use fixture_patch::*;
use console_server::reactor::Messages;
use console_server::clients::ResponseFilter;
use rust_dmx::{open_port, available_ports, Error as DmxPortError};
use wiggles_value::Datatype;

type GlobalAddress = (UniverseId, DmxAddress);

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchRequest {
    name: String,
    kind: String,
    address: Option<GlobalAddress>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ControlSourceDescription<S> {
    name: String,
    data_type: Datatype,
    source: Option<S>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PatchItemDescription<S> {
    id: FixtureId,
    name: String,
    kind: String,
    address: Option<GlobalAddress>,
    channel_count: DmxChannelCount,
    control_sources: Vec<ControlSourceDescription<S>>,
}

impl<'a, S: Clone> From<&'a PatchItem<S>> for PatchItemDescription<S> {
    fn from(item: &'a PatchItem<S>) -> Self {
        let control_sources =
            item.controls().zip(item.control_sources().iter())
                .map(|(control, source)| {
                    ControlSourceDescription {
                        name: control.name().to_string(),
                        data_type: control.data_type(),
                        source: source.clone(),
                    }
                })
                .collect();

        PatchItemDescription {
            id: item.id(),
            name: item.name.clone(),
            kind: item.kind().to_string(),
            address: item.global_address(),
            channel_count: item.channel_count(),
            control_sources: control_sources,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixtureKindDescription {
    name: String,
    channel_count: DmxChannelCount,
}

impl<'a> From<&'a Profile> for FixtureKindDescription {
    fn from(profile: &'a Profile) -> Self {
        FixtureKindDescription {
            name: profile.name().to_string(),
            channel_count: profile.channel_count(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnivWithPort {
    universe: UniverseId,
    port_namespace: String,
    port_id: String,
}

impl UnivWithPort {
    pub fn new<N, I>(id: UniverseId, namespace: N, port_id: I) -> Self
        where N: Into<String>, I: Into<String>
    {
        UnivWithPort {
            universe: id,
            port_namespace: namespace.into(),
            port_id: port_id.into(),
        }
    }
}

impl<'a> From<(UniverseId, &'a Universe)> for UnivWithPort {
    fn from((id, univ): (UniverseId, &Universe)) -> Self {
        UnivWithPort::new(id, univ.port_namespace(), univ.port_id())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatchServerRequest<S> {
    PatchState,
    NewPatches(Vec<PatchRequest>),
    Rename(FixtureId, String),
    Repatch(FixtureId, Option<GlobalAddress>),
    Remove(FixtureId),
    GetKinds,
    AddUniverse,
    RemoveUniverse(UniverseId, bool),
    AttachPort(UnivWithPort),
    AvailablePorts,
    SetControlSource(FixtureId, usize, Option<S>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PatchServerResponse<S> {
    PatchState(Vec<PatchItemDescription<S>>, Vec<UnivWithPort>),
    NewPatches(Vec<PatchItemDescription<S>>),
    Update(PatchItemDescription<S>),
    Remove(FixtureId),
    Kinds(Vec<FixtureKindDescription>),
    UpdateUniverse(UnivWithPort),
    UniverseRemoved(UniverseId),
    AvailablePorts(Vec<(String, String)>),
}

/// Handle a command to the fixture patch, producing either a response message or forwarding an
/// error to be lifted into a global generic error type.
/// In the successful case, optionally also provide a override that will be applied to the outgoing
/// responses.
pub fn handle_message<S: Clone>(
        patch: &mut Patch<S>,
        command: PatchServerRequest<S>)
        -> Result<(Messages<PatchServerResponse<S>>, Option<ResponseFilter>), PatchRequestError>
{
    use PatchServerRequest::*;
    use ResponseFilter::All;
    match command {
        PatchState => {
            let descriptions = patch.items().iter().map(Into::into).collect();
            let universes = patch.universes().iter().map(|item| (*item).into()).collect();
            Ok((Messages::one(PatchServerResponse::PatchState(descriptions, universes)), None))
        }
        NewPatches(mut reqs) => {
            // Keep track of fixture IDs that we've added so we can remove them if any patch action
            // encounters an error.
            let mut added_ids = Vec::new();
            let mut error = None;
            for req in reqs.drain(..) {
                // make sure this is a profile we know about
                match PROFILES.get(req.kind.as_str()) {
                    None => {
                        error = Some(PatchRequestError::ProfileNotFound(req.kind));
                        break;
                    }
                    Some(profile) => {
                        // got the profile, now try to patch it
                        match req.address {
                            Some((u, a)) => {
                                match patch.add_at_address(profile, Some(req.name), u, a) {
                                    Ok(id) => {
                                        added_ids.push(id);
                                    }
                                    Err(e) => {
                                        error = Some(e.into());
                                        break;
                                    }
                                }
                            }
                            None => {
                                added_ids.push(patch.add(profile, Some(req.name)));
                            }
                        }
                    }
                }
            }
            // if we encountered an error, remove all of the patches we added
            if let Some(e) = error {
                for id in added_ids {
                    patch.remove(id);
                }
                Err(e)
            }
            // Otherwise, generate descriptions of all of the patches we added
            else {
                let mut descriptions = Vec::new();
                for added_id in added_ids {
                    if let Ok(item) = patch.item(added_id) {
                        descriptions.push(item.into());
                    }
                }
                Ok((Messages::one(PatchServerResponse::NewPatches(descriptions)), Some(All)))
            }
        }
        Rename(id, name) => {
            let mut item = patch.item_mut(id)?;
            item.name = name;
            Ok((Messages::one(PatchServerResponse::Update((&*item).into())), Some(All)))
        }
        Repatch(id, addr) => {
            let item = match addr {
                Some((u, a)) => patch.repatch(id, u, a),
                None => patch.unpatch(id),
            }?;
            Ok((Messages::one(PatchServerResponse::Update(item.into())), Some(All)))
        }
        Remove(id) => {
            let item = patch.remove(id)?;
            Ok((Messages::one(PatchServerResponse::Remove(item.id())), Some(All)))
        }
        GetKinds => {
            let kinds = PROFILES.values().map(Into::into).collect();
            Ok((Messages::one(PatchServerResponse::Kinds(kinds)), None))
        }
        AddUniverse => {
            // add a universe mapped to an offline port
            let universe = Universe::new_offline();
            let namespace = universe.port_namespace().to_string();
            let port_id = universe.port_id().to_string();
            let univ_id = patch.add_universe(universe);
            let desc = UnivWithPort::new(univ_id, namespace, port_id);
            Ok((Messages::one(PatchServerResponse::UpdateUniverse(desc)), Some(All)))
        }
        RemoveUniverse(id, force) => {
            let removed_fixtures = patch.remove_universe(id, force)?;
            let mut messages = Messages::one(PatchServerResponse::UniverseRemoved(id));
            for id in removed_fixtures {
                if let Ok(item) = patch.item(id) {
                    messages.push(PatchServerResponse::Update(item.into()));
                }
            }
            Ok((messages, Some(All)))
        }
        AttachPort(pa) => {
            let port = open_port(&pa.port_namespace, pa.port_id.as_str())?;
            patch.set_universe_port(pa.universe, port)?;
            Ok((Messages::one(PatchServerResponse::UpdateUniverse(pa)), Some(All)))
        }
        AvailablePorts => {
            // get all the ports we have available
            Ok((Messages::one(PatchServerResponse::AvailablePorts(available_ports())), None))
        }
        SetControlSource(id, control_id, source) => {
            patch.set_control_source(id, control_id, source.clone())?;
            let item = patch.item(id)?;
            Ok((Messages::one(PatchServerResponse::Update(item.into())), Some(All)))
        }
    }
}

#[derive(Debug)]
pub enum PatchRequestError {
    PatchError(PatchError),
    ProfileNotFound(String),

}

impl fmt::Display for PatchRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PatchRequestError::PatchError(ref pe) => pe.fmt(f),
            PatchRequestError::ProfileNotFound(ref name) => write!(f, "Profile not found for fixture '{}'.", name),
        }
    }
}

impl From<PatchError> for PatchRequestError {
    fn from(pe: PatchError) -> Self {
        PatchRequestError::PatchError(pe)
    }
}

impl From<DmxPortError> for PatchRequestError {
    fn from(pe: DmxPortError) -> Self {
        PatchError::PortError(pe).into()
    }
}

impl std::error::Error for PatchRequestError {
    fn description(&self) -> &str {
        match *self {
            PatchRequestError::PatchError(ref pe) => pe.description(),
            PatchRequestError::ProfileNotFound(_) => "Fixture profile not found.",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            PatchRequestError::PatchError(ref pe) => Some(pe),
            PatchRequestError::ProfileNotFound(_) => None,
        }
    }
}