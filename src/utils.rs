pub fn clamp_to_i8(value: i32) -> i8 {
    if value < i8::MIN as i32 {
        i8::MIN
    } else if value > i8::MAX as i32 {
        i8::MAX
    } else {
        value as i8
    }
}
