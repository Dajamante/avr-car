#![no_std]
// don't do standard make for main
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;
use arduino_uno::prelude::*;

use crate::motors::{go_backward, go_forward, stop, turn_left, turn_right};
use crate::sensor::return_distance;

mod motors;
// lib std adds a layer to build the usual functions
mod sensor;

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

    let mut timer1 = dp.TC1; //make the timer avaible
    timer1.tccr1b.write(|w| w.cs1().prescale_64());

    let timer2 = dp.TC2;
    timer2.tccr2b.write(|w| w.cs2().prescale_1024());
    timer2.tccr2a.write(|w| w.wgm2().pwm_fast().com2b().match_clear());

    let mut trig = pins.d12.into_output(&mut pins.ddr);
    // floating input by default
    let mut echo = pins.d11;

    let mut _servo = pins.d3.into_output(&mut pins.ddr);

    let left_forw = pins.d4.into_output(&mut pins.ddr).downgrade();
    let left_back = pins.d5.into_output(&mut pins.ddr).downgrade();
    let right_forw = pins.d6.into_output(&mut pins.ddr).downgrade();
    let right_back = pins.d7.into_output(&mut pins.ddr).downgrade();
    let mut wheels = [left_forw, left_back, right_forw, right_back];


    'outer: loop {
        timer2.ocr2b.write(|x| unsafe { x.bits(20) });
        go_forward(&mut wheels);
        let value = return_distance(&mut trig, &mut echo, &mut timer1);
        ufmt::uwriteln!( & mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();

        if value < 10 {
            loop {
                stop(&mut wheels);

                timer2.ocr2b.write(|x| unsafe { x.bits(10) });
                let value_right = return_distance(&mut trig, &mut echo, &mut timer1);
                ufmt::uwriteln!( & mut serial, "On right, we are {} cms away from target!\r", value).void_unwrap();

                delay.delay_ms(500u16);

                timer2.ocr2b.write(|x| unsafe { x.bits(30) });
                let value_left = return_distance(&mut trig, &mut echo, &mut timer1);
                ufmt::uwriteln!( & mut serial, "On left, we are {} cms away from target!\r", value).void_unwrap();

                delay.delay_ms(500u16);

                if (value_left > value_right) && value_left > 20 {
                    turn_left(&mut wheels);

                } else if (value_right > value_left) && value_right > 20 {
                    turn_right(&mut wheels);

                } else {
                    go_backward(&mut wheels);
                    delay.delay_ms(500u16);
                    turn_right(&mut wheels);

                }
                continue 'outer;
            }
        }
        while timer1.tcnt1.read().bits() < 25000 {}

        ufmt::uwriteln!( & mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
    }
}

