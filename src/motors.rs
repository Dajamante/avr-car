use arduino_uno::hal::port;
use arduino_uno::prelude::*;

pub fn go_forward(
    left_forw: &mut arduino_uno::hal::port::portd::PD4<port::mode::Output>,
    left_back: &mut arduino_uno::hal::port::portd::PD5<port::mode::Output>,
    right_forw: &mut arduino_uno::hal::port::portd::PD6<port::mode::Output>,
    right_back: &mut arduino_uno::hal::port::portd::PD7<port::mode::Output>) {
    left_forw.set_high().void_unwrap();
    right_forw.set_high().void_unwrap();
    left_back.set_low().void_unwrap();
    right_back.set_low().void_unwrap();
}

pub fn go_backward(
    left_forw: &mut arduino_uno::hal::port::portd::PD4<port::mode::Output>,
    left_back: &mut arduino_uno::hal::port::portd::PD5<port::mode::Output>,
    right_forw: &mut arduino_uno::hal::port::portd::PD6<port::mode::Output>,
    right_back: &mut arduino_uno::hal::port::portd::PD7<port::mode::Output>) {
    left_forw.set_low().void_unwrap();
    right_forw.set_low().void_unwrap();
    left_back.set_high().void_unwrap();
    right_back.set_high().void_unwrap();
}


pub fn turn_righ(
    left_forw: &mut arduino_uno::hal::port::portd::PD4<port::mode::Output>,
    left_back: &mut arduino_uno::hal::port::portd::PD5<port::mode::Output>,
    right_forw: &mut arduino_uno::hal::port::portd::PD6<port::mode::Output>,
    right_back: &mut arduino_uno::hal::port::portd::PD7<port::mode::Output>,
) {
    let mut delay = arduino_uno::Delay::new();

    left_back.set_low().void_unwrap();
    right_back.set_low().void_unwrap();
    left_forw.set_low().void_unwrap();
    right_forw.set_low().void_unwrap();

    left_forw.set_high().void_unwrap();
    delay.delay_ms(500 as u16);
    right_forw.set_low().void_unwrap();
}

pub fn turn_left(
    left_forw: &mut arduino_uno::hal::port::portd::PD4<port::mode::Output>,
    left_back: &mut arduino_uno::hal::port::portd::PD5<port::mode::Output>,
    right_forw: &mut arduino_uno::hal::port::portd::PD6<port::mode::Output>,
    right_back: &mut arduino_uno::hal::port::portd::PD7<port::mode::Output>,
) {
    let mut delay = arduino_uno::Delay::new();

    left_back.set_low().void_unwrap();
    right_back.set_low().void_unwrap();
    left_forw.set_low().void_unwrap();
    right_forw.set_low().void_unwrap();

    right_forw.set_high().void_unwrap();
    delay.delay_ms(500 as u16);
    left_forw.set_low().void_unwrap();
}
