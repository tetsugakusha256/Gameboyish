use crate::{
    bus::Bus,
    util::cartridge_util::{load, CartridgeData, MBCType, CARTRIDGE_TYPE},
};
use std::{cell::RefCell, rc::Rc};

struct Cartridge {
    mbc_type: MBCType,
    data: CartridgeData,
    pub bus: Rc<RefCell<Bus>>,
}
impl Cartridge {
    fn new(bus: Rc<RefCell<Bus>>, file_path: &str) -> Cartridge {
        let data = load(file_path).unwrap();
        Cartridge {
            mbc_type: data.get_mbc_type(),
            data,
            bus,
        }
    }
    // fn switch
}
