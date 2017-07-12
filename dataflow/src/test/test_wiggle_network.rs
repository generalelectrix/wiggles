use network::Network;
use clocks::clock::{ClockValue, ClockProvider, ClockId};
use wiggles_value::Unipolar;
use wiggles::trial::{TestWiggle, CLASS as TEST_CLASS};
use wiggles::new_wiggle;
use wiggles::wiggle::{WiggleProvider, WiggleNetwork};
use serde_json;

struct TestClockProvider {}

impl ClockProvider for TestClockProvider {
    fn get_value(&self, _: ClockId) -> ClockValue {
        ClockValue::from_float_value(0.3, false)
    }
}

#[test]
fn test_wiggle_network() {
    let mut network: WiggleNetwork = Network::new();
    let wiggle = new_wiggle(TEST_CLASS, "test wiggle").unwrap();
    let wid = {
        let (wid, _) = network.add(wiggle);
        wid
    };

    let val = network.get_value(wid, 0u32.into(), 0.0, None, &TestClockProvider{});

    // check serialization/deserialization mechanism
    let ser_net = serde_json::to_string(&network).unwrap();
    let de_net: WiggleNetwork = serde_json::from_reader(ser_net.as_bytes()).unwrap();
    assert_eq!(network, de_net);
}