use crate::minimal_machines::test_machine::api::TestMachineNamespace;
use crate::minimal_machines::test_machine::TestMachine;
use smol::block_on;
use std::time::Instant;

use crate::{
    get_ethercat_device, validate_no_role_duplicates, validate_same_machine_identification_unique,
    MachineNewHardware, MachineNewParams, MachineNewTrait,
};

use anyhow::Error;
use ethercat_hal::devices::el1008::{EL1008Port, EL1008, EL1008_IDENTITY_A};
use ethercat_hal::devices::el2008::{EL2008Port, EL2008, EL2008_IDENTITY_A, EL2008_IDENTITY_B};
use ethercat_hal::io::{digital_input::DigitalInput, digital_output::DigitalOutput};

//Imports For Wago
/*
use ethercat_hal::devices::wago_750_354::{WAGO_750_354_IDENTITY_A, Wago750_354};
use ethercat_hal::devices::{EthercatDevice, downcast_device};
use smol::lock::RwLock;
use std::sync::Arc;
was das fur ein scheiß ist das hier
*/

impl MachineNewTrait for TestMachine {
    fn new<'maindevice>(params: &MachineNewParams) -> Result<Self, Error> {
        // validate general stuff
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
                    "[{}::MachineNewTrait/TestMachine::new] MachineNewHardware is not Ethercat",
                    module_path!()
                ));
            }
        };
        block_on(async {
            /*
            // Example usage of a Wago Coupler and a 750-1506 in the first slot, where the Output Port 1,2,3,4 is used
            let _wago_750_354 = get_ethercat_device::<Wago750_354>(
                hardware,
                params,
                0,
                [WAGO_750_354_IDENTITY_A].to_vec(),
            )
            .await?;

            let modules = Wago750_354::initialize_modules(_wago_750_354.1).await?;

            let mut coupler = _wago_750_354.0.write().await;

            for module in modules {
                coupler.set_module(module);
            }

            coupler.init_slot_modules(_wago_750_354.1);
            let dev = coupler.slot_devices.get(1).unwrap().clone().unwrap();
            let wago750_1506: Arc<RwLock<Wago750_1506>> =
                downcast_device::<Wago750_1506>(dev).await?;
            let do1 = DigitalOutput::new(wago750_1506.clone(), Wago750_1506OutputPort::DO1);
            let do2 = DigitalOutput::new(wago750_1506.clone(), Wago750_1506OutputPort::DO2);
            let do3 = DigitalOutput::new(wago750_1506.clone(), Wago750_1506OutputPort::DO3);
            let do4 = DigitalOutput::new(wago750_1506.clone(), Wago750_1506OutputPort::DO4);
            drop(coupler);
            */

            let el2008_1 = get_ethercat_device::<EL2008>(
                hardware,
                params,
                1,
                [EL2008_IDENTITY_A, EL2008_IDENTITY_B].to_vec(),
            )
            .await?
            .0;
            let el2008_2 = get_ethercat_device::<EL2008>(
                hardware,
                params,
                2,
                [EL2008_IDENTITY_A, EL2008_IDENTITY_B].to_vec(),
            )
            .await?
            .0;
            let el2008_3 = get_ethercat_device::<EL2008>(
                hardware,
                params,
                3,
                [EL2008_IDENTITY_A, EL2008_IDENTITY_B].to_vec(),
            )
            .await?
            .0;
            let el2008_4 = get_ethercat_device::<EL2008>(
                hardware,
                params,
                4,
                [EL2008_IDENTITY_A, EL2008_IDENTITY_B].to_vec(),
            )
            .await?
            .0;
            let el1008_1 =
                get_ethercat_device::<EL1008>(hardware, params, 5, [EL1008_IDENTITY_A].to_vec())
                    .await?
                    .0;
            let el1008_2 =
                get_ethercat_device::<EL1008>(hardware, params, 6, [EL1008_IDENTITY_A].to_vec())
                    .await?
                    .0;

            let douts = [
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO1),
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO2),
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO3),
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO4),
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO5),
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO6),
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO7),
                DigitalOutput::new(el2008_1.clone(), EL2008Port::DO8),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO1),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO2),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO3),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO4),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO5),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO6),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO7),
                DigitalOutput::new(el2008_2.clone(), EL2008Port::DO8),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO1),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO2),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO3),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO4),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO5),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO6),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO7),
                DigitalOutput::new(el2008_3.clone(), EL2008Port::DO8),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO1),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO2),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO3),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO4),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO5),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO6),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO7),
                DigitalOutput::new(el2008_4.clone(), EL2008Port::DO8),
            ];

            let dins = [
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI1),
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI2),
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI3),
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI4),
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI5),
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI6),
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI7),
                DigitalInput::new(el1008_1.clone(), EL1008Port::DI8),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI1),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI2),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI3),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI4),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI5),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI6),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI7),
                DigitalInput::new(el1008_2.clone(), EL1008Port::DI8),
            ];

            let (sender, receiver) = smol::channel::unbounded();
            let mut my_test = Self {
                api_receiver: receiver,
                api_sender: sender,
                machine_identification_unique: params.get_machine_identification_unique(),
                namespace: TestMachineNamespace {
                    namespace: params.namespace.clone(),
                },
                last_state_emit: Instant::now(),
                digital_output: [false; 32],
                main_sender: params.main_thread_channel.clone(),
                douts,
                dins,
            };
            my_test.emit_state();
            Ok(my_test)
        })
    }
}
