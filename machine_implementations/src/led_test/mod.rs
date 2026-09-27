mod act;
mod api;
mod events;
mod new;

pub use events::{LiveValuesEvent, StateEvent};

use crate::{MACHINE_LED_TEST, MachineMessage, QiTechMachine, VENDOR_QITECH, el1014::EL1014};
use control_core::socketio::namespace::NamespaceCacheingLogic;
use events::{LedTestEvents, LedTestNamespace};
use qitech_lib::{
    ethercat_hal::{
        devices::beckhoff_modules::el2004::EL2004, io::digital_output::DigitalOutputDevice,
    },
    machines::{MachineIdentification, MachineIdentificationUnique},
};
use std::{cell::RefCell, rc::Rc, time::Instant};
use tokio::sync::mpsc::{Receiver, Sender};

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
}

impl QiTechMachine for LedTestMachine {}
