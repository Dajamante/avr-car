//! ### The Motors Module
//! Handles the movement functions.
//! It unpacks the wheel pins in an array.

use arduino_uno::prelude::*;
const TURNING_TIME: u16 = 700u16;

/// The mutable wheels array is destructured for easier manipulation.
pub fn go_forward(
    wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4],
) {
    // Be careful here with the order of unpacking. In my case, pin 4 is connected to left forward, 5 to left backwards, etc
    let [left_forw, left_back, right_forw, right_back] = wheels;
    left_forw.set_high().void_unwrap();
    right_forw.set_high().void_unwrap();

    left_back.set_low().void_unwrap();
    right_back.set_low().void_unwrap();
}

pub fn go_backward(
    wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4],
) {
    let [left_forw, left_back, right_forw, right_back] = wheels;

    left_forw.set_low().void_unwrap();
    right_forw.set_low().void_unwrap();

    left_back.set_high().void_unwrap();
    right_back.set_high().void_unwrap();
}

pub fn turn_right(
    wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4],
) {
    stop(wheels);
    let [left_forw, _, _, _] = wheels;

    let mut delay = arduino_uno::Delay::new();
    left_forw.set_high().void_unwrap();
    delay.delay_ms(TURNING_TIME);
}
pub fn turn_left(
    wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4],
) {
    stop(wheels);
    let [_, _, right_forw, _] = wheels;

    let mut delay = arduino_uno::Delay::new();
    right_forw.set_high().void_unwrap();
    delay.delay_ms(TURNING_TIME);
}

pub fn stop(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    let [left_forw, left_back, right_forw, right_back] = wheels;

    left_forw.set_low().void_unwrap();
    left_back.set_low().void_unwrap();
    right_forw.set_low().void_unwrap();
    right_back.set_low().void_unwrap();
}
