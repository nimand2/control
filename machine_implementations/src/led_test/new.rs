use super::{LedTestMachine, events::LedTestNamespace};
use crate::{MachineHardware, MachineNew, el1014::EL1014};
use qitech_lib::ethercat_hal::devices::beckhoff_modules::{ek1100::EK1100, el2004::EL2004};
use std::time::Instant;

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
