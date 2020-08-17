//! ### The Motors Module
//! Handles the movement functions.
//! It unpacks the wheel pins in an array.

use arduino_uno::prelude::*;
const TURNING_TIME: u16 = 500u16;

/// The mutable wheels array is destructured for easier manipulation.
pub fn go_forward<>(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    let [&mut left_forw, &mut left_back, &mut right_forw, &mut right_back] = wheels;
    left_forw.set_high().void_unwrap();
    right_forw.set_high().void_unwrap();

    left_back.set_low().void_unwrap();
    right_back.set_low().void_unwrap();
}

pub fn go_backward(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    let [&mut left_forw, &mut left_back, &mut right_forw, &mut right_back] = wheels;

    left_forw.set_low().void_unwrap();
    right_forw.set_low().void_unwrap();

    left_back.set_high().void_unwrap();
    right_back.set_high().void_unwrap();
}


pub fn turn_right(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    stop(wheels);
    let [&mut left_forw, _, _, _] = wheels;

    let mut delay = arduino_uno::Delay::new();
    left_forw.set_high().void_unwrap();
    delay.delay_ms(TURNING_TIME);

}
pub fn turn_left(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    stop(wheels);
    let [_, _, &mut right_forw, _] = wheels;

    let mut delay = arduino_uno::Delay::new();
    right_forw.set_high().void_unwrap();
    delay.delay_ms(TURNING_TIME);

}

pub fn stop(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    let [&mut left_forw, &mut left_back, &mut right_forw, &mut right_back] = wheels;

    left_forw.set_low().void_unwrap();
    left_back.set_low().void_unwrap();
    right_forw.set_low().void_unwrap();
    right_back.set_low().void_unwrap();
}