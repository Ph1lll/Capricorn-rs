use vex_rt::prelude::*;

const VOLT_CONST: f32 = 12000.0 / 127.0;

fn to_volts(value: i8) -> i32 {
    (value as i32 * 12000 / 127) as i32
}

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
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),
            left_back: left_back_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),

            // Right side motors
            right_front: right_front_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),
            right_middle: right_middle_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),
            right_back: right_back_port
                .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                .unwrap(),
        }
    }

    pub fn move_volt(&mut self, left: i8, right: i8) {
        // Move left side
        self.left_front.move_voltage(to_volts(left)).unwrap();
        self.left_middle.move_voltage(to_volts(left)).unwrap();
        self.left_back.move_voltage(to_volts(left)).unwrap();

        // Move right side
        self.right_front.move_voltage(to_volts(left)).unwrap();
        self.right_middle.move_voltage(to_volts(left)).unwrap();
        self.right_back.move_voltage(to_volts(left)).unwrap();
    }
}