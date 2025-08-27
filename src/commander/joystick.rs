//! Joystick control mode >:)

pub mod joystick;

use crate::commander::Commander;
use crate::commander::op::Op;

use evdev::Device;
use joystick::{Axis, AxisEvent};
use std::thread;
use std::time::Duration;

const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc215;

const DEADZONE_LOW: i32 = 450;
const DEADZONE_HIGH: i32 = 574;
const MAX_STEPS: u32 = 50;

impl Commander {
    /// Entire commander's process when it uses a joystick instead
    pub fn joystick_process(&mut self) {
        let mut device = find_joystick(VENDOR_ID, PRODUCT_ID).expect("Joystick not found");
        println!(
            "Opened device: {}",
            device.name().unwrap_or("Unnamed device")
        );

        loop {
            for event in device.fetch_events().expect("Failed to fetch events") {
                if let Ok(axis_event) = AxisEvent::try_from(event) {
                    match axis_event {
                        AxisEvent::Axis(Axis::StickLeftRight, value) => {
                            let (direction, steps) = if value < DEADZONE_LOW {
                                (false, ((DEADZONE_LOW - value) as u32 / 10).min(MAX_STEPS))
                            } else if value > DEADZONE_HIGH {
                                (true, ((value - DEADZONE_HIGH) as u32 / 10).min(MAX_STEPS))
                            } else {
                                (false, 0)
                            };

                            let cmd = if direction {
                                Op::Left(100, 1)
                            } else {
                                Op::Right(100, 1)
                            };

                            self.send(&[cmd]);
                        }

                        _ => {}
                    }
                }
            }

            thread::sleep(Duration::from_micros(1));
        }
    }
}

fn find_joystick(vendor_id: u16, product_id: u16) -> Option<Device> {
    evdev::enumerate()
        .find(|(_, device)| {
            let id = device.input_id();
            id.vendor() == vendor_id && id.product() == product_id
        })
        .map(|(_, device)| device)
}
