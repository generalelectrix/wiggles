//! Tests for the fixture patch.
use super::*;

#[test]
fn test_universe_create_and_delete() {
    let mut patch = Patch::new();
    assert!(patch.describe_universes().is_empty());

    let u = Universe::new_offline("test universe 1");
    patch.add_universe(u);
    assert_eq!(1, patch.describe_universes().len());

    assert!(patch.universe(0).is_ok());
    assert_eq!(PatchError::InvalidUniverseId(1), patch.universe(1).unwrap_err());
}