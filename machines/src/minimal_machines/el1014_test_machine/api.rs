use std::sync::Arc;

use control_core::socketio::{
    event::{Event, GenericEvent},
    namespace::{
        CacheFn, CacheableEvents, Namespace, NamespaceCacheingLogic, cache_first_and_last_event,
    },
};
use serde::{Deserialize, Serialize};

use crate::{MachineApi, minimal_machines::el1014_test_machine::EL1014TestMachine};

#[derive(Serialize, Debug, Clone)]
pub struct StateEvent {
    pub inputs: [bool; 4],
}

impl StateEvent {
    pub fn build(&self) -> Event<Self> {
        Event::new("StateEvent", self.clone())
    }
}

pub enum EL1014TestMachineEvents {
    State(Event<StateEvent>),
}

#[derive(Deserialize)]
#[serde(tag = "action", content = "value")]
pub enum Mutation {}

#[derive(Debug, Clone)]
pub struct EL1014TestMachineNamespace {
    pub namespace: Option<Namespace>,
}

impl NamespaceCacheingLogic<EL1014TestMachineEvents> for EL1014TestMachineNamespace {
    fn emit(&mut self, events: EL1014TestMachineEvents) {
        let event = Arc::new(events.event_value());
        let buffer_fn = events.event_cache_fn();
        if let Some(ns) = &mut self.namespace {
            ns.emit(event, &buffer_fn);
        }
    }
}

impl CacheableEvents<EL1014TestMachineEvents> for EL1014TestMachineEvents {
    fn event_value(&self) -> GenericEvent {
        match self {
            EL1014TestMachineEvents::State(event) => event.clone().into(),
        }
    }

    fn event_cache_fn(&self) -> CacheFn {
        cache_first_and_last_event()
    }
}

impl MachineApi for EL1014TestMachine {
    fn api_get_sender(&self) -> smol::channel::Sender<crate::MachineMessage> {
        self.api_sender.clone()
    }

    fn api_mutate(&mut self, _value: serde_json::Value) -> Result<(), anyhow::Error> {
        // EL1014 is a digital input terminal - no mutations supported
        Ok(())
    }

    fn api_event_namespace(&mut self) -> Option<Namespace> {
        self.namespace.namespace.clone()
    }
}
