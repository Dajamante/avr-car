
//! ### The sensor Module
//! The sensor module takes a SensorUnit value struct with a trigger, an echo
//! and a timer. As per HC-SR04 documentation, the trigger needs to be up 10 μs

use arduino_uno::prelude::*;
use arduino_uno::hal::port::mode::{Floating};

const TRIGGER_UP_TIME: u16 = 10u16;

/// struct sensor_unit is instantiated in main, as it needs
/// pins and timer.
pub struct SensorUnit {
    pub trig: arduino_uno::hal::port::portb::PB4<arduino_uno::hal::port::mode::Output>,
    pub echo: arduino_uno::hal::port::portb::PB3<arduino_uno::hal::port::mode::Input<Floating>>,
    pub timer: arduino_uno::atmega328p::TC1,
}

pub fn return_distance(sensor_unit: &mut SensorUnit) -> u16 {
    let mut delay = arduino_uno::Delay::new();
    // we are writing to the tcnt1 register:
    // https://docs.rs/avr-device/0.2.1/avr_device/atmega328p/tc1/tcnt1/type.W.html
    // when no special methods are listed, it is meant to use bits(), and
    // we can click on the W(rite) or R(ead) to see the implementation details.

    // Writing a value directly into a register is unsafe,
    // in case another register needs so you need to explicitely specify
    sensor_unit.timer.tcnt1.write(|w| unsafe { w.bits(0) });

    // void_unwrap() --> set high could return an error
    // we are using a crate named void + unwrap
    // if not, there will be a warning on the fact that result is not used
    sensor_unit.trig.set_high().void_unwrap();
    delay.delay_us(TRIGGER_UP_TIME);
    sensor_unit.trig.set_low().void_unwrap();

    while sensor_unit.echo.is_low().void_unwrap() {
        // if more than 200 ms ( = 50000) we might have not detected anything and can continue.
        if sensor_unit.timer.tcnt1.read().bits() >= 65000 {
            return 63500;
        }
    }

    // restarting the timer by writing 0 bits to it
    sensor_unit.timer.tcnt1.write(|w| unsafe { w.bits(0) });

    // waiting for the echo to get low again
    while sensor_unit.echo.is_high().void_unwrap() {}

    // Taking the time the echo was high, which is as long as the time the signal was out.
    // 1 timer count == 4 us so * 4 to get a value in microsecs
    // we divide by 58 to get the distance in cm, since (34000 cm/s * 1e-6 us time)/2 (back and forth measurement)
    // == 0.017 more or less 1/58
    let value = (sensor_unit.timer.tcnt1.read().bits() * 4) / 58;

    // !! AVR only natively supports 8 and 16 bit integers, so *do not* return bigger
    u16::from(value)

}