//! Tests for the fixture patch.
use super::*;
use super::profiles::dimmer::PROFILE as dimmer_profile;

#[test]
fn test_universe_create_and_delete() {
    let mut patch = Patch::new();
    assert!(patch.describe_universes().is_empty());

    let u = Universe::new_offline("test universe 0");
    patch.add_universe(u);
    assert_eq!(1, patch.describe_universes().len());

    assert!(patch.universe(0).is_ok());
    assert_eq!(PatchError::InvalidUniverseId(1), patch.universe(1).unwrap_err());
    patch.remove_universe(0, false).unwrap();
    assert_eq!(PatchError::InvalidUniverseId(0), patch.universe(0).unwrap_err());
}

#[test]
fn test_no_remove_universe_with_fixtures() {
    let mut patch = Patch::new();
    patch.add_universe(Universe::new_offline("test universe 0"))
}