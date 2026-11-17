//! Integration tests for SigmaOS-ambient-light-sensors.

use sigmaos_ambient_light_sensors_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
