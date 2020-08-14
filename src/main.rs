#![no_std]
// don't do standard make for main
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut delay = arduino_uno::Delay::new();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let timer2 = dp.TC2;
    timer2.tccr2b.write(|w| w.cs2().prescale_1024());
    timer2.tccr2a.write(|w| w.wgm2().pwm_fast().com2b().match_clear());


    let mut _servo = pins.d3.into_output(&mut pins.ddr);

    'outer: loop {
        timer2.ocr2b.write(|x| unsafe { x.bits(10) });
        delay.delay_ms(2000u16);
        timer2.ocr2b.write(|x| unsafe { x.bits(30) });
        delay.delay_ms(2000u16);
        timer2.ocr2b.write(|x| unsafe { x.bits(20) });
        delay.delay_ms(2000u16);
    }
}

