use super::{EthercatDeviceProcessing, NewEthercatDevice, SubDeviceIdentityTuple};
use crate::helpers::ethercrab_types::EthercrabSubDevicePreoperational;
use crate::io::digital_input::{DigitalInputDevice, DigitalInputInput};
use crate::pdo::{PredefinedPdoAssignment, TxPdo, basic::BoolPdoObject};
use ethercat_hal_derive::{EthercatDevice, TxPdo};

/// EL1014 4-channel digital input device
///
/// 24V DC, 10us filter
#[derive(Clone, EthercatDevice)]
pub struct EL1014 {
    pub txpdo: EL1014TxPdo,
    is_used: bool,
}

impl EthercatDeviceProcessing for EL1014 {}

impl std::fmt::Debug for EL1014 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EL1014")
    }
}

impl NewEthercatDevice for EL1014 {
    fn new() -> Self {
        Self {
            txpdo: EL1014TxPdo::default(),
            is_used: false,
        }
    }
}

impl DigitalInputDevice<EL1014Port> for EL1014 {
    fn get_input(&self, port: EL1014Port) -> Result<DigitalInputInput, anyhow::Error> {
        let error = anyhow::anyhow!(
            "[{}::Device::digital_input_state] Port {:?} is not available",
            module_path!(),
            port
        );
        Ok(DigitalInputInput {
            value: match port {
                EL1014Port::DI1 => self.txpdo.channel1.as_ref().ok_or(error)?.value,
                EL1014Port::DI2 => self.txpdo.channel2.as_ref().ok_or(error)?.value,
                EL1014Port::DI3 => self.txpdo.channel3.as_ref().ok_or(error)?.value,
                EL1014Port::DI4 => self.txpdo.channel4.as_ref().ok_or(error)?.value,
            },
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EL1014Port {
    DI1,
    DI2,
    DI3,
    DI4,
}

impl EL1014Port {
    pub const fn to_bit_index(&self) -> usize {
        match self {
            Self::DI1 => 0,
            Self::DI2 => 1,
            Self::DI3 => 2,
            Self::DI4 => 3,
        }
    }
}

#[derive(Debug, Clone, TxPdo)]
pub struct EL1014TxPdo {
    #[pdo_object_index(0x1A00)]
    pub channel1: Option<BoolPdoObject>,
    #[pdo_object_index(0x1A01)]
    pub channel2: Option<BoolPdoObject>,
    #[pdo_object_index(0x1A02)]
    pub channel3: Option<BoolPdoObject>,
    #[pdo_object_index(0x1A03)]
    pub channel4: Option<BoolPdoObject>,
}

impl Default for EL1014TxPdo {
    fn default() -> Self {
        Self {
            channel1: Some(BoolPdoObject::default()),
            channel2: Some(BoolPdoObject::default()),
            channel3: Some(BoolPdoObject::default()),
            channel4: Some(BoolPdoObject::default()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EL1014PredefinedPdoAssignment {
    All,
}

impl PredefinedPdoAssignment<EL1014TxPdo, ()> for EL1014PredefinedPdoAssignment {
    fn txpdo_assignment(&self) -> EL1014TxPdo {
        match self {
            Self::All => EL1014TxPdo {
                channel1: Some(BoolPdoObject::default()),
                channel2: Some(BoolPdoObject::default()),
                channel3: Some(BoolPdoObject::default()),
                channel4: Some(BoolPdoObject::default()),
            },
        }
    }

    fn rxpdo_assignment(&self) {
        unreachable!()
    }
}

pub const EL1014_VENDOR_ID: u32 = 0x2;
pub const EL1014_PRODUCT_ID: u32 = 0x03f63052;
pub const EL1014_REVISION_A: u32 = 0x00110000;
pub const EL1014_REVISION_B: u32 = 0x00120000;

pub const EL1014_IDENTITY_A: SubDeviceIdentityTuple =
    (EL1014_VENDOR_ID, EL1014_PRODUCT_ID, EL1014_REVISION_A);
pub const EL1014_IDENTITY_B: SubDeviceIdentityTuple =
    (EL1014_VENDOR_ID, EL1014_PRODUCT_ID, EL1014_REVISION_B);
