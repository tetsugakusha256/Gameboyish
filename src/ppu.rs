use std::{rc::Rc, cell::RefCell};

use crate::{bus::Bus, screen::Screen};

pub struct PPU{
    pub bus: Rc<RefCell<Bus>>,
}
impl PPU {
   pub fn next_tick(&self){

   }
   fn draw(){

   }
}
