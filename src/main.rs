#![no_std]
// don't do standard make for main
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::prelude::*;

use crate::motors::{go_backward, go_forward, stop, turn_left};

mod motors;// lib std adds a layer to build the usual functions

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut delay = arduino_uno::Delay::new();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    let timer1 = dp.TC1; //make the timer avaible
    timer1.tccr1b.write(|w| w.cs1().prescale_64());

    let mut trig = pins.d12.into_output(&mut pins.ddr);
    // floating input by default
    let echo = pins.d11;
    let mut left_wheel_forward = pins.d4.into_output(&mut pins.ddr);
    let mut left_wheel_backward = pins.d5.into_output(&mut pins.ddr);
    let mut right_wheel_forward = pins.d6.into_output(&mut pins.ddr);
    let mut right_wheel_backward = pins.d7.into_output(&mut pins.ddr);


    'outer: loop {
        go_forward(
            &mut left_wheel_forward,
            &mut left_wheel_backward,
            &mut right_wheel_forward,
            &mut right_wheel_backward,
        );

        timer1.tcnt1.write(|w| unsafe { w.bits(0) });
        // warning that I don't use the result --> void_unwrap
        trig.set_high().void_unwrap();
        delay.delay_us(10u16);
        trig.set_low().void_unwrap();

        while echo.is_low().void_unwrap() {
            if timer1.tcnt1.read().bits() >= 50000 {
                ufmt::uwriteln!(&mut serial, "Nothing was detected and jump to outer loop.\r").void_unwrap();
                continue 'outer;
            }
        }

        //restarting the timer
        timer1.tcnt1.write(|w| unsafe { w.bits(0) });

        // wait for the echo to get low again
        while echo.is_high().void_unwrap() {}


        let value = (timer1.tcnt1.read().bits() * 4) / 58;

        if value < 10 {
            loop {
                ufmt::uwriteln!(&mut serial, "Going backwards.\r").void_unwrap();
                
                go_backward(
                    &mut left_wheel_forward,
                    &mut left_wheel_backward,
                    &mut right_wheel_forward,
                    &mut right_wheel_backward,
                );
                stop(
                    &mut left_wheel_forward,
                    &mut left_wheel_backward,
                    &mut right_wheel_forward,
                    &mut right_wheel_backward,
                );
                ufmt::uwriteln!(&mut serial, "Turning left.\r").void_unwrap();
                turn_left(
                    &mut left_wheel_forward,
                    &mut left_wheel_backward,
                    &mut right_wheel_forward,
                    &mut right_wheel_backward,
                );
                ufmt::uwriteln!(&mut serial, "Just turned left.\r").void_unwrap();
                ufmt::uwriteln!(&mut serial, "Continue to outer loop.\r").void_unwrap();
                continue 'outer;
            }
        }

        while timer1.tcnt1.read().bits() < 25000 {}

        ufmt::uwriteln!(&mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
    }
}

/*
type PinD4 = arduino_uno::port::portd::PD4<Output>;
type PinD5 = arduino_uno::port::portd::PD5<Output>;
type PinD6 = arduino_uno::port::portd::PD6<Output>;
type PinD7 = arduino_uno::port::portd::PD7<Output>;


fn init_wheels() ->(PinD4, PinD5, PinD6, PinD7){
    //wheels are input
    let mut left_wheel_forward = pins.d4.into_output(&mut pins.ddr);
    let mut left_wheel_backward = pins.d5.into_output(&mut pins.ddr);
    let mut right_wheel_forward = pins.d6.into_output(&mut pins.ddr);
    let mut right_wheel_backward = pins.d7.into_output(&mut pins.ddr);

    (left_wheel_forward, left_wheel_backward, right_wheel_forward, right_wheel_backward)
}

#[arduino_uno::entry]
fn main() -> ! {

    struct Resources {
        left_wheel_forward: PinD4,
        left_wheel_backward: PinD5,
        right_wheel_forward: PinD6,
        right_wheel_backward: PinD7,
    } = init_wheels();

 */