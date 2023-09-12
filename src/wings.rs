use vex_rt::prelude::*;

pub struct Wings {
    pub left_wing: AdiDigitalOutput,
    pub right_wing: AdiDigitalOutput,
}

impl Wings {
    pub fn new(left_wing_port: AdiPort, right_wing_port: AdiPort) -> Self {
        Self {
            left_wing: left_wing_port.into_adi_digital_output().unwrap(),
            right_wing: right_wing_port.into_adi_digital_output().unwrap(),
        }
    }
}
