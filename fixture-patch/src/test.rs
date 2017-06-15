//! Tests for the fixture patch.
use super::*;
use super::profiles::dimmer::PROFILE as dimmer_profile;
use super::profiles::clay_paky_astroraggi_power::PROFILE as astro_profile;
use wiggles_value::*;

fn assert_fixture_patched_at(p: &Patch, id: FixtureId, address: Option<(UniverseId, DmxAddress)>) {
    assert_eq!(address, p.item(id).unwrap().address);
}

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
    let fid = patch.add_at_address(&dimmer_profile, None, uid, 1).unwrap();
    assert_eq!(PatchError::NonEmptyUniverse(uid), patch.remove_universe(uid, false).unwrap_err());
    assert_fixture_patched_at(&patch, fid, Some((uid, 1)));
    // Add another universe and a fixture in it, to ensure universe removal unpatching does affect
    // others.
    let uid_other = patch.add_universe(Universe::new_offline("test universe 1"));
    let fid_other = patch.add_at_address(&dimmer_profile, None, uid_other, 1).unwrap();
    assert_fixture_patched_at(&patch, fid_other, Some((uid_other, 1)));
    // Force universe removal; it should unpatch the fixture in it.
    patch.remove_universe(uid, true).unwrap();
    assert_fixture_patched_at(&patch, fid, None);
    assert_fixture_patched_at(&patch, fid_other, Some((uid_other, 1)));
}

#[test]
fn test_no_out_of_range_address() {
    let mut patch = Patch::new();
    let uid = patch.add_universe(Universe::new_offline("test universe 0"));
    let bad_addr = 600;
    assert_eq!(
        PatchError::InvalidDmxAddress(bad_addr),
        patch.add_at_address(&dimmer_profile, None, uid, bad_addr).unwrap_err());
}

#[test]
fn test_render() {
    let mut patch = Patch::new();
    let uid = patch.add_universe(Universe::new_offline("test universe 0"));
    let fid = patch.add_at_address(&dimmer_profile, None, uid, 1).unwrap();
    fn assert_all_zeros(patch: &Patch, uid: UniverseId) {
        assert_eq!([0; 512][..], patch.universe(uid).unwrap().buffer[..]);
    }
    assert_all_zeros(&patch, uid);
    patch.item_mut(fid).unwrap().set_control(0, Data::Unipolar(Unipolar(1.0)));
    let errs = patch.render();
    assert!(errs.is_empty());
    {
        let univ = patch.universe(uid).unwrap();
        assert_eq!(255, univ.buffer[0]);
        assert_eq!([0; 511][..], univ.buffer[1..]);
    }
    // Make sure inactive fixtures don't render.
    patch.set_active(fid, false).unwrap();
    let errs = patch.render();
    assert!(errs.is_empty());
    assert_all_zeros(&patch, uid);

}

#[test]
fn test_serde() {
    // make a patch with a couple of universes and a couple of different fixture types
    let mut patch = Patch::new();
    let uid0 = patch.add_universe(Universe::new_offline("test universe 0"));
    let uid1 = patch.add_universe(Universe::new_offline("test universe 1"));
    let fid0 = patch.add_at_address(&dimmer_profile, None, uid0, 124).unwrap();
    let fid1 = patch.add_at_address(&dimmer_profile, None, uid0, 127).unwrap();
    let fid2 = patch.add_at_address(&dimmer_profile, None, uid0, 511).unwrap();
    let fid3 = patch.add_at_address(&astro_profile, None, uid1, 127).unwrap();
    let fid4 = patch.add_at_address(&dimmer_profile, None, uid1, 1).unwrap();
    let fid5 = patch.add(&astro_profile, None);
    
    // serialize to json
    let json_patch = serde_json::to_string(&patch).unwrap();
    // round-trip
    let json_round_trip_patch: Patch = serde_json::from_str(&json_patch).unwrap();
    assert_eq!(patch, json_round_trip_patch);

    // serialize to bincode
    let bincode_patch = bincode::serialize(&patch, bincode::Infinite).unwrap();
    // round-trip
    let bincode_round_trip_patch = bincode::deserialize(&bincode_patch).unwrap();
    assert_eq!(patch, bincode_round_trip_patch);
}