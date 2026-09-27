use control_core::socketio::{
    event::{Event, GenericEvent},
    namespace::{
        CacheFn, CacheableEvents, Namespace, NamespaceCacheingLogic, cache_first_and_last_event,
    },
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StateEvent {
    pub outputs: [bool; 8],
}

impl StateEvent {
    pub(super) fn build(&self) -> Event<Self> {
        Event::new("StateEvent", self.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LiveValuesEvent {
    pub inputs: [bool; 8],
}

impl LiveValuesEvent {
    pub(super) fn build(&self) -> Event<Self> {
        Event::new("LiveValuesEvent", self.clone())
    }
}

pub(super) enum LedTestEvents {
    State(Event<StateEvent>),
    LiveValues(Event<LiveValuesEvent>),
}

#[derive(Debug)]
pub(super) struct LedTestNamespace {
    pub(super) namespace: Option<Namespace>,
}

impl NamespaceCacheingLogic<LedTestEvents> for LedTestNamespace {
    fn emit(&mut self, events: LedTestEvents) {
        let event = Arc::new(events.event_value());
        let buffer_fn = events.event_cache_fn();
        if let Some(namespace) = &mut self.namespace {
            namespace.emit(event, &buffer_fn);
        }
    }
}

impl CacheableEvents<LedTestEvents> for LedTestEvents {
    fn event_value(&self) -> GenericEvent {
        match self {
            Self::State(event) => event.clone().into(),
            Self::LiveValues(event) => event.clone().into(),
        }
    }

    fn event_cache_fn(&self) -> CacheFn {
        cache_first_and_last_event()
    }
}
