use std::time::Instant;

use control_core::socketio::namespace::NamespaceCacheingLogic;
use ethercat_hal::io::digital_input::DigitalInput;
use smol::channel::{Receiver, Sender};

use self::api::{EL1014TestMachineEvents, EL1014TestMachineNamespace, StateEvent};
use crate::{
    AsyncThreadMessage, EL1014_TEST_MACHINE, Machine, MachineMessage, VENDOR_QITECH,
    machine_identification::{MachineIdentification, MachineIdentificationUnique},
};

pub mod act;
pub mod api;
pub mod new;

#[derive(Debug)]
pub struct EL1014TestMachine {
    pub api_receiver: Receiver<MachineMessage>,
    pub api_sender: Sender<MachineMessage>,
    pub machine_identification_unique: MachineIdentificationUnique,
    pub main_sender: Option<Sender<AsyncThreadMessage>>,
    pub namespace: EL1014TestMachineNamespace,
    pub last_state_emit: Instant,
    pub inputs: [bool; 4],
    pub digital_input: [DigitalInput; 4],
}

impl Machine for EL1014TestMachine {
    fn get_machine_identification_unique(&self) -> MachineIdentificationUnique {
        self.machine_identification_unique.clone()
    }

    fn get_main_sender(&self) -> Option<Sender<AsyncThreadMessage>> {
        self.main_sender.clone()
    }
}

impl EL1014TestMachine {
    pub const MACHINE_IDENTIFICATION: MachineIdentification = MachineIdentification {
        vendor: VENDOR_QITECH,
        machine: EL1014_TEST_MACHINE,
    };

    pub fn get_state(&self) -> StateEvent {
        StateEvent {
            inputs: self.inputs,
        }
    }

    pub fn emit_state(&mut self) {
        for (i, di) in self.digital_input.iter().enumerate() {
            self.inputs[i] = match di.get_value() {
                Ok(v) => v,
                Err(_) => false,
            };
        }

        let event = StateEvent {
            inputs: self.inputs,
        }
        .build();
        self.namespace.emit(EL1014TestMachineEvents::State(event));
    }
}
