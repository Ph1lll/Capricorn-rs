use vex_rt::prelude::*;

const VOLT_CONST: f32 = 12000.0 / 127.0;

pub struct Drivetrain {
    pub left_front: Motor,
    pub left_middle: Motor,
    pub left_back: Motor,
    pub right_front: Motor,
    pub right_middle: Motor,
    pub right_back: Motor,
}

impl Drivetrain {
    pub fn new(
        left_front_port: SmartPort,
        left_middle_port: SmartPort,
        left_back_port: SmartPort,
        right_front_port: SmartPort,
        right_middle_port: SmartPort,
        right_back_port: SmartPort,
    ) -> Self {
        Self {
            // Left side motors
            left_front: left_front_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),
            left_middle: left_middle_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, true)
                .unwrap(),
            left_back: left_back_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),

            // Right side motors
            right_front: right_front_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, true)
                .unwrap(),
            right_middle: right_middle_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),
            right_back: right_back_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, true)
                .unwrap(),
        }
    }

    pub fn run(&mut self, power: i8, rotate: i8) {
        let left = ((power as f32 + rotate as f32).clamp(-127.0, 127.0) * VOLT_CONST) as i32;
        let right = ((power as f32 - rotate as f32).clamp(-127.0, 127.0) * VOLT_CONST) as i32;

        // Move left side
        self.left_front.move_voltage(left).unwrap();
        self.left_middle.move_voltage(left).unwrap();
        self.left_back.move_voltage(left).unwrap();

        // Move right side
        self.right_front.move_voltage(right).unwrap();
        self.right_middle.move_voltage(right).unwrap();
        self.right_back.move_voltage(right).unwrap();
    }
}
