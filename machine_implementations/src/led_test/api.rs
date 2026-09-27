use super::{LedTestMachine, LiveValuesEvent, StateEvent};
use crate::{MachineApi, MachineMessage, MachineValues};
use control_core::socketio::namespace::Namespace;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

#[derive(Deserialize)]
#[serde(tag = "action", content = "value")]
enum Mutation {
    SetOutput { index: usize, on: bool },
    SetAllOutputs { on: bool },
}

impl LedTestMachine {
    pub(super) fn act_machine_message(&mut self, message: MachineMessage) {
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
