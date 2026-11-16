// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ambient-light-sensors: ambient light sensors component model.
//!
//! Real implementation: sensors, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sensor {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmbientLightSensorsError {
    EmptySensorName,
    DuplicateSensor(String),
    EmptyEventSubject,
}

impl fmt::Display for AmbientLightSensorsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySensorName => write!(f, "sensor name must not be empty"),
            Self::DuplicateSensor(id) => write!(f, "duplicate sensor {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AmbientLightSensorsError {}

pub struct AmbientLightSensors {
    pub name: String,
    sensors: BTreeMap<String, Sensor>,
    events: BTreeMap<String, Event>,
}

impl AmbientLightSensors {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sensors: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_sensor(&mut self, item: Sensor) -> Result<(), AmbientLightSensorsError> {
        if item.name.trim().is_empty() {
            return Err(AmbientLightSensorsError::EmptySensorName);
        }
        let id = item.id.clone();
        if self.sensors.contains_key(&id) {
            return Err(AmbientLightSensorsError::DuplicateSensor(id.clone()));
        }
        self.sensors.insert(id, item);
        Ok(())
    }

    pub fn sensor_count(&self) -> usize {
        self.sensors.len()
    }

    pub fn get_sensor(&self, id: &str) -> Option<&Sensor> {
        self.sensors.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AmbientLightSensorsError> {
        if event.subject.trim().is_empty() {
            return Err(AmbientLightSensorsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sensor(id: &str, name: &str) -> Sensor {
        Sensor {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AmbientLightSensors::new("AmbientLightSensors");
        assert_eq!(m.name, "AmbientLightSensors");
        assert_eq!(m.sensor_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_sensor() {
        let mut m = AmbientLightSensors::new("AmbientLightSensors");
        m.add_sensor(sample_sensor("a1", "Alpha")).unwrap();
        assert_eq!(m.sensor_count(), 1);
        assert!(m.get_sensor("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AmbientLightSensors::new("AmbientLightSensors");
        assert!(m.add_sensor(sample_sensor("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AmbientLightSensors::new("AmbientLightSensors");
        m.add_sensor(sample_sensor("x", "One")).unwrap();
        assert!(m.add_sensor(sample_sensor("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AmbientLightSensors::new("AmbientLightSensors");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
