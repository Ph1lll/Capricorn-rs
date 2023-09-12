use vex_rt::prelude::*;

const START_POINT: f64 = 50.0;
const RELEASE_POINT: f64 = 250.0;
const CURRENT_THRESHOLD: i32 = 2400;

pub struct Catapult {
    pub left_motor: Motor,
    pub right_motor: Motor,
}

impl Catapult {
    pub fn new(left_motor_port: SmartPort, right_motor_port: SmartPort) -> Self {
        Self {
            left_motor: left_motor_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::EncoderTicks, true)
                .unwrap(),
            right_motor: right_motor_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::EncoderTicks, false)
                .unwrap(),
        }
    }

    pub fn set_brake_mode(&mut self) {
        self.left_motor.set_brake_mode(BrakeMode::Hold).unwrap();
        self.right_motor.set_brake_mode(BrakeMode::Hold).unwrap();
    }

    fn get_position(&self) -> f64 {
        (self.left_motor.get_position().unwrap() + self.right_motor.get_position().unwrap()) / 2.0
    }

    fn get_current(&self) -> i32 {
        (self.left_motor.get_current_draw().unwrap() + self.right_motor.get_current_draw().unwrap())
            / 2
    }

    pub fn is_at_start(&self) -> bool {
        self.get_position() <= START_POINT
    }

    pub fn is_at_end(&self) -> bool {
        self.get_position() >= RELEASE_POINT
    }

    pub fn has_triball(&self) -> bool {
        self.get_current() >= CURRENT_THRESHOLD
    }

    pub fn fire(&mut self) {
        self.left_motor.move_voltage(12000).unwrap();
        self.right_motor.move_voltage(12000).unwrap();
    }
    pub fn go_back(&mut self) {
        self.left_motor.move_absolute(START_POINT, 100).unwrap();
        self.right_motor.move_absolute(START_POINT, 100).unwrap();
    }
}
