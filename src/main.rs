#![no_std]
#![no_main]
use core::time::Duration;
use vex_rt::{prelude::*, select};

mod drivetrain;
mod utils;

struct Capricorn {
    drive: Mutex<drivetrain::Drivetrain>,
    controller: Controller,
}

impl Robot for Capricorn {
    fn new(peripherals: Peripherals) -> Self {
        Self {
            drive: Mutex::new(drivetrain::Drivetrain::new(
                peripherals.port01,
                peripherals.port02,
                peripherals.port03,
                peripherals.port04,
                peripherals.port05,
                peripherals.port06,
            )),
            controller: peripherals.master_controller,
        }
    }

    fn initialize(self: &mut Capricorn, _ctx: Context) {
        // Do any extra initialization here.
    }

    fn autonomous(self: &mut Capricorn, _ctx: Context) {
        println!("autonomous");
        // Write your autonomous routine here.
    }

    fn opcontrol(self: &mut Capricorn, ctx: Context) {
        println!("opcontrol");

        let mut l = Loop::new(Duration::from_millis(20));
        loop {
            let power: i32 = self.controller.left_stick.get_y().unwrap().into();
            let rotate: i32 = self.controller.right_stick.get_x().unwrap().into();

            // Update the motors.
            self.drive.lock().move_volt(
                utils::clamp_to_i8(power + rotate),
                utils::clamp_to_i8(power - rotate),
            );

            select! {
                // If the driver control period is done, break out of the loop.
                _ = ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => continue,
            }
        }
    }

    fn disabled(self: &mut Capricorn, _ctx: Context) {
        println!("disabled");
        // This runs when the robot is in disabled mode.
    }
}

entry!(Capricorn);
