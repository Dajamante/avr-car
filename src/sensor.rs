use arduino_uno::prelude::*;
use arduino_uno::hal::port::mode::{Floating};

pub fn return_distance(
    trig:& mut arduino_uno::hal::port::portb::PB4<arduino_uno::hal::port::mode::Output>,
    echo: &mut arduino_uno::hal::port::portb::PB3<arduino_uno::hal::port::mode::Input<Floating>>,
    timer1: &mut arduino_uno::atmega328p::TC1
) -> u16 {
    let mut delay = arduino_uno::Delay::new();

    timer1.tcnt1.write(|w| unsafe { w.bits(0) });
    // warning that I don't use the result --> void_unwrap
    trig.set_high().void_unwrap();
    delay.delay_us(10u16);
    trig.set_low().void_unwrap();

    'outer: while echo.is_low().void_unwrap() {
        // more than 200 ms ( = 50000)
        if timer1.tcnt1.read().bits() >= 65000 {
            continue 'outer;
        }
    }

    //restarting the timer
    timer1.tcnt1.write(|w| unsafe { w.bits(0) });

    // wait for the echo to get low again
    while echo.is_high().void_unwrap() {}

    let value = (timer1.tcnt1.read().bits() * 4) / 58;

    u16::from(value)

}