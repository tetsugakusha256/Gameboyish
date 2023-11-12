use std::{cell::RefCell, rc::Rc};

use crate::bus::Bus;

pub struct IOHandler{
    pub bus: Rc<RefCell<Bus>>,
}
impl IOHandler{
    pub fn next_tick(&self){
        //TODO: should update the bus according to the input read
    }
}
