use std::{sync::Arc, time::Instant};

use anyhow::Error;
use ethercat_hal::{
    devices::{
        EthercatDevice,
        el1014::{EL1014, EL1014_IDENTITY_A, EL1014_IDENTITY_B, EL1014Port},
    },
    io::digital_input::DigitalInput,
};
use smol::{block_on, lock::RwLock};

use super::{EL1014TestMachine, api::EL1014TestMachineNamespace};
use crate::{
    MachineNewHardware, MachineNewParams, MachineNewTrait, get_ethercat_device,
    validate_no_role_duplicates, validate_same_machine_identification_unique,
};

impl MachineNewTrait for EL1014TestMachine {
    fn new<'maindevice>(params: &MachineNewParams) -> Result<Self, Error> {
        let device_identification = params
            .device_group
            .iter()
            .map(|device_identification| device_identification.clone())
            .collect::<Vec<_>>();
        validate_same_machine_identification_unique(&device_identification)?;
        validate_no_role_duplicates(&device_identification)?;

        let hardware = match &params.hardware {
            MachineNewHardware::Ethercat(x) => x,
            _ => {
                return Err(anyhow::anyhow!(
                    "[{}::EL1014TestMachine::new] MachineNewHardware is not Ethercat",
                    module_path!()
                ));
            }
        };

        block_on(async {
            // Role 0: Beckhoff EL1014 terminal
            let (el1014, _subdevice): (Arc<RwLock<EL1014>>, _) = get_ethercat_device::<EL1014>(
                hardware,
                params,
                0,
                [EL1014_IDENTITY_A, EL1014_IDENTITY_B].to_vec(),
            )
            .await?;

            let di1 = DigitalInput::new(el1014.clone(), EL1014Port::DI1);
            let di2 = DigitalInput::new(el1014.clone(), EL1014Port::DI2);
            let di3 = DigitalInput::new(el1014.clone(), EL1014Port::DI3);
            let di4 = DigitalInput::new(el1014.clone(), EL1014Port::DI4);

            let (sender, receiver) = smol::channel::unbounded();
            let mut machine = Self {
                api_receiver: receiver,
                api_sender: sender,
                machine_identification_unique: params.get_machine_identification_unique(),
                namespace: EL1014TestMachineNamespace {
                    namespace: params.namespace.clone(),
                },
                last_state_emit: Instant::now(),
                inputs: [false; 4],
                main_sender: params.main_thread_channel.clone(),
                digital_input: [di1, di2, di3, di4],
            };
            machine.emit_state();
            Ok(machine)
        })
    }
}
