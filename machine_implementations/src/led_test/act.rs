use super::LedTestMachine;
use qitech_lib::{
    ethercat_hal::io::digital_input::DigitalInputDevice,
    machines::{Machine, MachineDataRegistry, MachineError, MachineIdentificationUnique},
};

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
