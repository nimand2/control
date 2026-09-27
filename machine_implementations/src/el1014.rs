use bitvec::{order::Lsb0, slice::BitSlice};
use qitech_lib::ethercat_hal::{
    devices::{
        EthercatDevice, EthercatDeviceProcessing, EthercatDeviceUsed, Module, NewEthercatDevice,
    },
    io::digital_input::DigitalInputDevice,
};

#[derive(Debug)]
pub struct EL1014 {
    inputs: [bool; 4],
    used: bool,
}

impl NewEthercatDevice for EL1014 {
    fn new() -> Self {
        Self {
            inputs: [false; 4],
            used: false,
        }
    }
}

impl EthercatDeviceProcessing for EL1014 {}

impl EthercatDeviceUsed for EL1014 {
    fn is_used(&self) -> bool {
        self.used
    }

    fn set_used(&mut self, used: bool) {
        self.used = used;
    }
}

impl EthercatDevice for EL1014 {
    fn input(&mut self, input: &BitSlice<u8, Lsb0>) -> Result<(), anyhow::Error> {
        if input.len() < self.inputs.len() {
            anyhow::bail!("EL1014 expected 4 input bits, received {}", input.len());
        }
        for (channel, value) in self.inputs.iter_mut().enumerate() {
            *value = input[channel];
        }
        Ok(())
    }

    fn input_len(&self) -> usize {
        self.inputs.len()
    }

    fn output(&self, _output: &mut BitSlice<u8, Lsb0>) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn output_len(&self) -> usize {
        0
    }

    fn into_any_boxed(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_module(&self) -> bool {
        false
    }

    fn get_module(&self) -> Option<Module> {
        None
    }

    fn set_module(&mut self, _module: Module) {}
}

impl DigitalInputDevice for EL1014 {
    fn get_input(&self, port: usize) -> Result<bool, anyhow::Error> {
        self.inputs
            .get(port)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("EL1014 has 4 ports (0-3), requested {port}"))
    }

    fn get_port_count(&self) -> usize {
        self.inputs.len()
    }
}
