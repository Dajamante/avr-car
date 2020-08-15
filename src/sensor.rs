use arduino_uno::prelude::*;
use arduino_uno::hal::port::mode::{Floating};
use crate::SensorUnit;

pub fn return_distance(sensor_unit: &mut SensorUnit) -> u16 {
    let mut delay = arduino_uno::Delay::new();
    // we are writing to the tcnt1 register.
    // https://docs.rs/avr-device/0.2.1/avr_device/atmega328p/tc1/tcnt1/type.W.html
    // when no special methods are listed, it is meant to use bits
    // we can click on the W or R to see the details

    // also, writing a value directly into a register is unsafe, in case another register needs it
    // you need to indicate it to the compiler
    sensor_unit.timer.tcnt1.write(|w| unsafe { w.bits(0) });
    // void_unwrap() --> set high could return an error
    // unwrap --> quick panic on error
    // we are using a crate named void
    // if not, there will be a warning on the fact that result is not used

    // as per the datasheet, the trigger needs to be up 10 us
    sensor_unit.trig.set_high().void_unwrap();
    delay.delay_us(10u16);
    sensor_unit.trig.set_low().void_unwrap();

    'outer: while sensor_unit.echo.is_low().void_unwrap() {
        // if more than 200 ms ( = 50000) we might have not detected
        // anything
        if sensor_unit.timer.tcnt1.read().bits() >= 65000 {
            continue 'outer;
        }
    }

    // restarting the timer
    sensor_unit.timer.tcnt1.write(|w| unsafe { w.bits(0) });

    // waiting for the echo to get low again
    while sensor_unit.echo.is_high().void_unwrap() {}

    // 1 timer count == 4 us so * 4 to get a value in microsecs
    // we divide by 58 to get the distance in cm, since (34000 cm/s * 1e-6 us time)/2 (back and forth measurement)
    // == 0.017 more or less 1/58
    let value = (sensor_unit.timer.tcnt1.read().bits() * 4) / 58;

    // !! AVR only natively supports 8 and 16 bit integers, so do not return bigger
    u16::from(value)

}