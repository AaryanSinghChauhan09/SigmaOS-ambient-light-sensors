// Integration test for SigmaOS-ambient-light-sensors
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use ambient_light_sensors as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
