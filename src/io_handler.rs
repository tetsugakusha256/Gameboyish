extern crate evdev;
use std::{cell::RefCell, rc::Rc};

use crate::bus::Bus;
use evdev::{Device, EventType, Key};

pub struct IOHandler {
    pub bus: Rc<RefCell<Bus>>,
    device: Device,
}
impl IOHandler {
    pub fn new(bus: Rc<RefCell<Bus>>) -> IOHandler {
        let path = "/dev/input/event16"; // Replace X with the appropriate event number
        let device = Device::open(path).expect("Failed to create device");
        IOHandler { bus, device }
    }
    pub fn next_tick(&mut self) {
        //TODO: Run this in a separate thread to avoid blocking the emulator
        if let Ok(events) = self.device.fetch_events() {
            for event in events {
                if event.event_type() == EventType::KEY {
                    match event.value() {
                        1 => {
                            println!("Press");
                            println!("event : {}", event.code())
                        }
                        _ => {}
                    }
                    if event.code() == Key::KEY_END.code() {}
                    if event.code() == Key::KEY_0.code() {}
                }
            }
        }
    }
}

fn tick() {
    loop {}
}
