use crate::el1014::EL1014;
use qitech_lib::ethercat_hal::{
    MetaSubdevice,
    devices::{EthercatDevice, NewEthercatDevice, device_from_subdevice_identity_rc},
};
use std::{cell::RefCell, rc::Rc};

const BECKHOFF_VENDOR_ID: u32 = 0x0000_0002;
const EL1014_PRODUCT_ID: u32 = 0x03f6_3052;

pub fn device_from_subdevice(
    meta: &MetaSubdevice,
) -> Result<Rc<RefCell<dyn EthercatDevice>>, anyhow::Error> {
    if meta.vendor == BECKHOFF_VENDOR_ID && meta.product_id == EL1014_PRODUCT_ID {
        return Ok(Rc::new(RefCell::new(EL1014::new())));
    }

    device_from_subdevice_identity_rc(meta)
}
