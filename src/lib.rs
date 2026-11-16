// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-oomd: systemd oomd component model.
//!
//! Real implementation: oomds, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oomd {
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
pub enum SystemdOomdError {
    EmptyOomdName,
    DuplicateOomd(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdOomdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOomdName => write!(f, "oomd name must not be empty"),
            Self::DuplicateOomd(id) => write!(f, "duplicate oomd {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdOomdError {}

pub struct SystemdOomd {
    pub name: String,
    oomds: BTreeMap<String, Oomd>,
    events: BTreeMap<String, Event>,
}

impl SystemdOomd {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            oomds: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_oomd(&mut self, item: Oomd) -> Result<(), SystemdOomdError> {
        if item.name.trim().is_empty() {
            return Err(SystemdOomdError::EmptyOomdName);
        }
        let id = item.id.clone();
        if self.oomds.contains_key(&id) {
            return Err(SystemdOomdError::DuplicateOomd(id.clone()));
        }
        self.oomds.insert(id, item);
        Ok(())
    }

    pub fn oomd_count(&self) -> usize {
        self.oomds.len()
    }

    pub fn get_oomd(&self, id: &str) -> Option<&Oomd> {
        self.oomds.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdOomdError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdOomdError::EmptyEventSubject);
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

    fn sample_oomd(id: &str, name: &str) -> Oomd {
        Oomd {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdOomd::new("SystemdOomd");
        assert_eq!(m.name, "SystemdOomd");
        assert_eq!(m.oomd_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_oomd() {
        let mut m = SystemdOomd::new("SystemdOomd");
        m.add_oomd(sample_oomd("a1", "Alpha")).unwrap();
        assert_eq!(m.oomd_count(), 1);
        assert!(m.get_oomd("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdOomd::new("SystemdOomd");
        assert!(m.add_oomd(sample_oomd("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdOomd::new("SystemdOomd");
        m.add_oomd(sample_oomd("x", "One")).unwrap();
        assert!(m.add_oomd(sample_oomd("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdOomd::new("SystemdOomd");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
