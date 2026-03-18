use crate::machine_identification::{MachineIdentification, MachineIdentificationUnique};
use crate::minimal_machines::test_machine::api::{StateEvent, TestMachineEvents};
use crate::{AsyncThreadMessage, Machine, MachineMessage};
use control_core::socketio::namespace::NamespaceCacheingLogic;
use ethercat_hal::io::{digital_input::DigitalInput, digital_output::DigitalOutput};
use smol::channel::{Receiver, Sender};
use std::time::Instant;
pub mod act;
pub mod api;
pub mod new;
use crate::minimal_machines::test_machine::api::TestMachineNamespace;
use crate::{TEST_MACHINE, VENDOR_QITECH};

#[derive(Debug)]
pub struct TestMachine {
    pub api_receiver: Receiver<MachineMessage>,
    pub api_sender: Sender<MachineMessage>,
    pub machine_identification_unique: MachineIdentificationUnique,
    pub namespace: TestMachineNamespace,
    pub last_state_emit: Instant,
    pub digital_output: [bool; 32],
    pub main_sender: Option<Sender<AsyncThreadMessage>>,
    pub douts: [DigitalOutput; 32],
    pub dins: [DigitalInput; 16],
}

impl Machine for TestMachine {
    fn get_machine_identification_unique(&self) -> MachineIdentificationUnique {
        self.machine_identification_unique.clone()
    }

    fn get_main_sender(&self) -> Option<Sender<AsyncThreadMessage>> {
        self.main_sender.clone()
    }
}
impl TestMachine {
    pub const MACHINE_IDENTIFICATION: MachineIdentification = MachineIdentification {
        vendor: VENDOR_QITECH,
        machine: TEST_MACHINE,
    };
}

impl TestMachine {
    pub fn get_state(&self) -> StateEvent {
        StateEvent {
            digital_output: self.digital_output,
            digital_input: self
                .dins
                .iter()
                .map(|input| input.get_value().unwrap_or(false))
                .collect::<Vec<_>>()
                .try_into()
                .expect("digital input vector into array[16] should work"),
        }
    }

    pub fn emit_state(&mut self) {
        let event = self.get_state().build();
        self.namespace.emit(TestMachineEvents::State(event));
    }

    pub fn set_output(&mut self, index: usize, on: bool) {
        if index < self.digital_output.len() {
            self.digital_output[index] = on;
            self.douts[index].set(on);
            self.emit_state();
        }
    }

    pub fn set_all_outputs(&mut self, on: bool) {
        self.digital_output = [on; 32];
        for output in &self.douts {
            output.set(on);
        }
        self.emit_state();
    }
}
