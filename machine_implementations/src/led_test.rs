use crate::{
    MACHINE_LED_TEST, MachineApi, MachineHardware, MachineMessage, MachineNew, MachineValues,
    QiTechMachine, VENDOR_QITECH, el1014::EL1014,
};
use control_core::socketio::{
    event::{Event, GenericEvent},
    namespace::{
        CacheFn, CacheableEvents, Namespace, NamespaceCacheingLogic, cache_first_and_last_event,
    },
};
use qitech_lib::{
    ethercat_hal::{
        devices::beckhoff_modules::{ek1100::EK1100, el2004::EL2004},
        io::{digital_input::DigitalInputDevice, digital_output::DigitalOutputDevice},
    },
    machines::{
        Machine, MachineDataRegistry, MachineError, MachineIdentification,
        MachineIdentificationUnique,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{cell::RefCell, rc::Rc, sync::Arc, time::Instant};
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StateEvent {
    pub outputs: [bool; 8],
}

impl StateEvent {
    fn build(&self) -> Event<Self> {
        Event::new("StateEvent", self.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LiveValuesEvent {
    pub inputs: [bool; 8],
}

impl LiveValuesEvent {
    fn build(&self) -> Event<Self> {
        Event::new("LiveValuesEvent", self.clone())
    }
}

enum LedTestEvents {
    State(Event<StateEvent>),
    LiveValues(Event<LiveValuesEvent>),
}

#[derive(Deserialize)]
#[serde(tag = "action", content = "value")]
enum Mutation {
    SetOutput { index: usize, on: bool },
    SetAllOutputs { on: bool },
}

#[derive(Debug)]
struct LedTestNamespace {
    namespace: Option<Namespace>,
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

pub struct LedTestMachine {
    identification: MachineIdentificationUnique,
    api_sender: Sender<MachineMessage>,
    api_receiver: Receiver<MachineMessage>,
    namespace: LedTestNamespace,
    inputs: [bool; 8],
    outputs: [bool; 8],
    input_devices: [Rc<RefCell<EL1014>>; 2],
    output_devices: [Rc<RefCell<EL2004>>; 2],
    last_live_values_emit: Instant,
}

impl LedTestMachine {
    pub const MACHINE_IDENTIFICATION: MachineIdentification = MachineIdentification {
        vendor: VENDOR_QITECH,
        machine: MACHINE_LED_TEST,
    };

    fn emit_state(&mut self) {
        self.namespace.emit(LedTestEvents::State(
            StateEvent {
                outputs: self.outputs,
            }
            .build(),
        ));
    }

    fn emit_live_values(&mut self) {
        self.namespace.emit(LedTestEvents::LiveValues(
            LiveValuesEvent {
                inputs: self.inputs,
            }
            .build(),
        ));
        self.last_live_values_emit = Instant::now();
    }

    fn set_output(&mut self, index: usize, on: bool) {
        if index >= self.outputs.len() {
            return;
        }
        self.outputs[index] = on;
        self.output_devices[index / 4]
            .borrow_mut()
            .set_output(index % 4, on);
        self.emit_state();
    }

    fn set_all_outputs(&mut self, on: bool) {
        for index in 0..self.outputs.len() {
            self.outputs[index] = on;
            self.output_devices[index / 4]
                .borrow_mut()
                .set_output(index % 4, on);
        }
        self.emit_state();
    }

    fn act_machine_message(&mut self, message: MachineMessage) {
        match message {
            MachineMessage::SubscribeNamespace(namespace) => {
                self.namespace.namespace = Some(namespace);
                self.emit_state();
                self.emit_live_values();
            }
            MachineMessage::UnsubscribeNamespace => self.namespace.namespace = None,
            MachineMessage::HttpApiJsonRequest(value) => {
                let _ = self.api_mutate(value);
            }
            MachineMessage::RequestValues(sender) => {
                let _ = sender.send(MachineValues {
                    state: serde_json::to_value(StateEvent {
                        outputs: self.outputs,
                    })
                    .expect("Failed to serialize LED test state"),
                    live_values: serde_json::to_value(LiveValuesEvent {
                        inputs: self.inputs,
                    })
                    .expect("Failed to serialize LED test inputs"),
                });
            }
        }
    }
}

impl MachineNew for LedTestMachine {
    fn new(hw: MachineHardware) -> Result<Self, anyhow::Error> {
        let _coupler = hw.try_get_ethercat_device_by_role::<EK1100>(0)?;
        let input_devices = [
            hw.try_get_ethercat_device_by_role::<EL1014>(1)?,
            hw.try_get_ethercat_device_by_role::<EL1014>(2)?,
        ];
        let output_devices = [
            hw.try_get_ethercat_device_by_role::<EL2004>(3)?,
            hw.try_get_ethercat_device_by_role::<EL2004>(4)?,
        ];
        let (api_sender, api_receiver) = tokio::sync::mpsc::channel(8);
        let mut machine = Self {
            identification: hw.identification,
            api_sender,
            api_receiver,
            namespace: LedTestNamespace { namespace: None },
            inputs: [false; 8],
            outputs: [false; 8],
            input_devices,
            output_devices,
            last_live_values_emit: Instant::now(),
        };
        machine.set_all_outputs(false);
        Ok(machine)
    }
}

impl Machine for LedTestMachine {
    fn act(&mut self, _registry: Option<&mut MachineDataRegistry>) -> Result<(), MachineError> {
        if let Ok(message) = self.api_receiver.try_recv() {
            self.act_machine_message(message);
        }

        for (device_index, device) in self.input_devices.iter().enumerate() {
            let device = device.borrow();
            for port in 0..4 {
                self.inputs[device_index * 4 + port] = device.get_input(port).unwrap_or(false);
            }
        }

        if self.last_live_values_emit.elapsed().as_millis() >= 33 {
            self.emit_live_values();
        }
        Ok(())
    }

    fn react(&mut self, _registry: &MachineDataRegistry) {}

    fn get_identification(&self) -> MachineIdentificationUnique {
        self.identification
    }
}

impl MachineApi for LedTestMachine {
    fn api_mutate(&mut self, request_body: Value) -> Result<(), anyhow::Error> {
        match serde_json::from_value(request_body)? {
            Mutation::SetOutput { index, on } => self.set_output(index, on),
            Mutation::SetAllOutputs { on } => self.set_all_outputs(on),
        }
        Ok(())
    }

    fn api_event_namespace(&mut self) -> Option<Namespace> {
        self.namespace.namespace.clone()
    }

    fn get_api_sender(&self) -> Sender<MachineMessage> {
        self.api_sender.clone()
    }

    fn act_machine_message(&mut self, message: MachineMessage) {
        LedTestMachine::act_machine_message(self, message);
    }
}

impl QiTechMachine for LedTestMachine {}
