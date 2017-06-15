//! Tests for the fixture patch.
use super::*;
use super::profiles::dimmer::PROFILE as dimmer_profile;

#[test]
fn test_universe_create_and_delete() {
    let mut patch = Patch::new();
    assert!(patch.describe_universes().is_empty());

    let u = Universe::new_offline("test universe 0");
    let uid = patch.add_universe(u);
    assert_eq!(1, patch.describe_universes().len());
    assert!(patch.universe(uid).is_ok());
    assert_eq!(PatchError::InvalidUniverseId(uid+1), patch.universe(uid+1).unwrap_err());
    patch.remove_universe(uid, false).unwrap();
    assert_eq!(PatchError::InvalidUniverseId(uid), patch.universe(uid).unwrap_err());
}

#[test]
fn test_no_remove_universe_with_fixtures() {
    let mut patch = Patch::new();
    let uid = patch.add_universe(Universe::new_offline("test universe 0"));
    let fid = patch.add_at_address(&dimmer_profile, None, uid, 0).unwrap();
    assert_eq!(PatchError::NonEmptyUniverse(uid), patch.remove_universe(uid, false).unwrap_err());
    // Add another universe and a fixture in it, to ensure universe removal unpatching does affect
    // others.
    let uid_other = patch.add_universe(Universe::new_offline("test universe 1"));
    let fid_other = patch.add_at_address(&dimmer_profile, None, uid_other, 0).unwrap();
    assert_eq!(Some((uid_other, 0)), patch.item(fid_other).unwrap().address);
    // Force universe removal; it should unpatch the fixture in it.
    patch.remove_universe(uid, true).unwrap();
    assert_eq!(None, patch.item(fid).unwrap().address);
    assert_eq!(Some((uid_other, 0)), patch.item(fid_other).unwrap().address);
}