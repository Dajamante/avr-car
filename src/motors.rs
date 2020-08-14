use arduino_uno::prelude::*;

pub fn go_forward<>(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    wheels[0].set_high().void_unwrap();
    wheels[2].set_high().void_unwrap();

    wheels[1].set_low().void_unwrap();
    wheels[3].set_low().void_unwrap();
}

pub fn go_backward(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    wheels[0].set_low().void_unwrap();
    wheels[2].set_low().void_unwrap();

    wheels[1].set_high().void_unwrap();
    wheels[3].set_high().void_unwrap();
}


pub fn turn_right(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    stop(wheels);
    let mut delay = arduino_uno::Delay::new();
    wheels[0].set_high().void_unwrap();
    delay.delay_ms(500 as u16);

}
pub fn turn_left(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    stop(wheels);
    let mut delay = arduino_uno::Delay::new();
    wheels[2].set_high().void_unwrap();
    delay.delay_ms(500 as u16);

}

pub fn stop(wheels: &mut [arduino_uno::hal::port::Pin<arduino_uno::hal::port::mode::Output>; 4]) {
    wheels[0].set_low().void_unwrap();
    wheels[1].set_low().void_unwrap();
    wheels[2].set_low().void_unwrap();
    wheels[3].set_low().void_unwrap();
}
