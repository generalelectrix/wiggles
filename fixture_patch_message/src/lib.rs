//! Message passing based API for the fixture patch.

extern crate fixture_patch;
extern crate console_server;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate rust_dmx;

use std::fmt;
use fixture_patch::*;
use console_server::reactor::Messages;
use rust_dmx::{DmxPort, open_port, available_ports, Error as DmxPortError};

type GlobalAddress = (UniverseId, DmxAddress);

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchRequest {
    name: String,
    kind: String,
    address: Option<GlobalAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchItemDescription {
    id: FixtureId,
    name: String,
    kind: String,
    address: Option<GlobalAddress>,
    channel_count: DmxChannelCount,
}

impl<'a> From<&'a PatchItem> for PatchItemDescription {
    fn from(item: &'a PatchItem) -> Self {
        PatchItemDescription {
            id: item.id(),
            name: item.name.clone(),
            kind: item.kind().to_string(),
            address: item.global_address(),
            channel_count: item.channel_count(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortAttachment {
    universe: UniverseId,
    port_namespace: String,
    port_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatchServerRequest {
    PatchState,
    NewPatches(Vec<PatchRequest>),
    Rename(FixtureId, String),
    Repatch(FixtureId, Option<GlobalAddress>),
    Remove(FixtureId),
    GetKinds,
    AddUniverse,
    RemoveUniverse(UniverseId, bool),
    AttachPort(PortAttachment),
    AvailablePorts,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatchServerResponse {
    PatchState(Vec<PatchItemDescription>),
    NewPatches(Vec<PatchItemDescription>),
    Update(PatchItemDescription),
    Remove(FixtureId),
    Kinds(Vec<FixtureKindDescription>),
    NewUniverse(UniverseId),
    UniverseRemoved(UniverseId),
    PortAttached(PortAttachment),
    AvailablePorts(Vec<(String, String)>),
}

/// Handle a command to the fixture patch, producing either a response message or forwarding an
/// error to be lifted into a global generic error type.
pub fn handle_message(
        patch: &mut Patch,
        command: PatchServerRequest)
        -> Result<PatchServerResponse, PatchRequestError>
{
    use PatchServerRequest::*;
    match command {
        PatchState => {
            let descriptions = patch.items().iter().map(Into::into).collect();
            Ok(PatchServerResponse::PatchState(descriptions))
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
                Ok(PatchServerResponse::NewPatches(descriptions))
            }
        }
        Rename(id, name) => {
            let mut item = patch.item_mut(id)?;
            item.name = name;
            Ok(PatchServerResponse::Update((&*item).into()))
        }
        Repatch(id, addr) => {
            let item = match addr {
                Some((u, a)) => patch.repatch(id, u, a),
                None => patch.unpatch(id),
            }?;
            Ok(PatchServerResponse::Update(item.into()))
        }
        Remove(id) => {
            let item = patch.remove(id)?;
            Ok(PatchServerResponse::Remove(item.id()))
        }
        GetKinds => {
            let kinds = PROFILES.values().map(Into::into).collect();
            Ok(PatchServerResponse::Kinds(kinds))
        }
        AddUniverse => {
            // add a universe mapped to an offline port
            Ok(PatchServerResponse::NewUniverse(patch.add_universe(Universe::new_offline())))
        }
        RemoveUniverse(id, force) => {
            patch.remove_universe(id, force)?;
            Ok(PatchServerResponse::UniverseRemoved(id))
        }
        AttachPort(pa) => {
            let port = open_port(&pa.port_namespace, pa.port_id.as_str())?;
            patch.set_universe_port(pa.universe, port)?;
            Ok(PatchServerResponse::PortAttached(pa))
        }
        AvailablePorts => {
            // get all the ports we have available
            Ok(PatchServerResponse::AvailablePorts(available_ports()))
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