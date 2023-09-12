#![no_std]
#![no_main]
use core::time::Duration;
use vex_rt::{prelude::*, select};

mod catapult;
mod drivetrain;
mod wings;

struct Capricorn {
    drive: Mutex<drivetrain::Drivetrain>,
    wings: Mutex<wings::Wings>,
    catapult: catapult::Catapult,
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
            wings: Mutex::new(wings::Wings::new(peripherals.port_a, peripherals.port_b)),
            catapult: catapult::Catapult::new(peripherals.port13, peripherals.port14),
            controller: peripherals.master_controller,
        }
    }

    fn initialize(self: &mut Capricorn, _ctx: Context) {
        self.catapult.set_brake_mode();
    }

    fn autonomous(self: &mut Capricorn, _ctx: Context) {
        println!("autonomous");
        // Write your autonomous routine here.
    }

    fn opcontrol(self: &mut Capricorn, ctx: Context) {
        println!("opcontrol");

        let mut l = Loop::new(Duration::from_millis(20));
        loop {
            // Drivetrain
            self.drive.lock().run(
                self.controller.left_stick.get_y().unwrap(),
                self.controller.right_stick.get_x().unwrap(),
            );

            // Wings
            self.wings
                .lock()
                .left_wing
                .write(self.controller.l2.is_pressed().unwrap())
                .unwrap();
            self.wings
                .lock()
                .right_wing
                .write(self.controller.r2.is_pressed().unwrap())
                .unwrap();

            // Catapult
            if self.catapult.is_at_start() && self.catapult.has_triball() {
                self.catapult.fire();
            } else if self.catapult.is_at_end() {
                self.catapult.go_back();
            }

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
