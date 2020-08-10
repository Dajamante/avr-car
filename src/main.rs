// lib std adds a layer to build the usual functions

#![no_std]
// don't do standard make for main
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::prelude::*;

// creates the main function
// attribute macro -> transforms the next as the entry point

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
    let mut echo = pins.d11;

    //wheels are input
    let mut left_wheel_forward = pins.d4.into_output(&mut pins.ddr);
    let mut left_wheel_backward = pins.d5.into_output(&mut pins.ddr);
    let mut right_wheel_forward = pins.d6.into_output(&mut pins.ddr);
    let mut right_wheel_backward = pins.d7.into_output(&mut pins.ddr);

    'outer: loop {

        left_wheel_forward.set_high().void_unwrap();
        right_wheel_forward.set_high().void_unwrap();
        left_wheel_backward.set_low().void_unwrap();
        right_wheel_backward.set_low().void_unwrap();

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
                delay.delay_ms(2000 as u16);
                left_wheel_forward.set_low().void_unwrap();
                right_wheel_forward.set_low().void_unwrap();
                left_wheel_backward.set_high().void_unwrap();
                right_wheel_backward.set_high().void_unwrap();
                delay.delay_ms(2000 as u16);
                ufmt::uwriteln!(&mut serial, "Continue to outer loop.\r").void_unwrap();
                continue 'outer;
            }
        }


// this loop waited for 200 ms
// we need to wait 100 ms as per docs (that recommend 60 ms)
        while timer1.tcnt1.read().bits() < 25000 {}

// check stuff on the screen screen /dev/tty.usbserial-14440 57600
// interrupt the screen CTRL A + K
        ufmt::uwriteln!(&mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
    }
}
