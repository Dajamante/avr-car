const SERVO_CENTER: u8 = 20;
const SERVO_RIGHT: u8 = 30;
const SERVO_LEFT: u8 = 10;

/// We use a generic for the pin
pub struct ServoUnit<S: embedded_hal::PwmPin<Duty = u8>> {
    pub servo: S,

}
/// We implement embedded_hal::PwmPin for the struct ServoUnit,
/// with rotations as methods and not lost functions
impl <S: embedded_hal::PwmPin<Duty = u8>> ServoUnit<S>{
    pub fn look_right(&mut self) { self.servo.set_duty(SERVO_RIGHT) }
    pub fn look_left(&mut self) { self.servo.set_duty(SERVO_LEFT) }
    pub fn look_front(&mut self) { self.servo.set_duty(SERVO_CENTER) }
}

