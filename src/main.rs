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
    // acquires a singleton of all the peripherals
    // everything inside the MCU
    // https://docs.rs/avr-device/0.2.1/avr_device/atmega328p/struct.Peripherals.html (raw registers abstraction)
    // TC are the timers
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut delay = arduino_uno::Delay::new();

    //all the ports are bunched into pins
    // all the pins https://rahix.github.io/avr-hal/arduino_uno/struct.Pins.html
    // all pins are configured as inputs and floating
    // no pull up register --> the state is undefined. You add a resistor to pull it up or down (5V in case of arduino)

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        // protocol to communicate bytes in 2 directions
        // USART0 is moved to serial, serial becomes the new owner
        //https://rahix.github.io/avr-hal/atmega328p_hal/usart/struct.Usart0.html
        dp.USART0,
        //
        // rx: receive pin (hardwired into the MCU)
        // tx : PD1 is the "hardcoded output"
        // the ownership is moved by writing explicitely
        // input, output is enforced at compile time,
        pins.d0,
        // d1 is converted  v---v into an output
        pins.d1.into_output(&mut pins.ddr),
        // choose well known baud rates (9600)
        57600,
    );

    let timer1 = dp.TC1; //make the timer avaible
    // initialisation so we write over everything and set prescaler ~> 262 ms approx
    timer1.tccr1b.write(|w| w.cs1().prescale_64());
    // the timer is running !

    let mut timer2 = arduino_uno::pwm::Timer2Pwm::new(dp.TC2);
    let mut pd3 = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer2);

    // Reset timer2

    // look front is 1.5 ms pulse
    // 0.0015 / (1/(16e6/64))
    pd3.set_duty((191) as u8);
    pd3.enable();

    // Digital pin 13 is also connected to an onboard LED marked "L"
    // moving the pins.d13 into 1. into_output 2. into led, it takes ddr register
    let mut led = pins.d13.into_output(&mut pins.ddr);
    let mut trig = pins.d12.into_output(&mut pins.ddr);
    // floating input by default
    let mut echo = pins.d11;

    //probably not needed
    //let mut echo = pins.d3.into_input(&mut pins.ddr);

    // void_unwrap() --> set high could return an error
    // unwrap --> quick panic on error

    //wheels are input
    let mut left_wheel_forward = pins.d4.into_output(&mut pins.ddr);
    let mut left_wheel_backward = pins.d5.into_output(&mut pins.ddr);
    let mut right_wheel_forward = pins.d6.into_output(&mut pins.ddr);
    let mut right_wheel_backward = pins.d7.into_output(&mut pins.ddr);

    'outer: loop {
        //https://docs.rs/avr-device/0.2.1/avr_device/atmega328p/tc1/tcnt1/type.W.html
        // no special methods, means use bits
        // click on the W or R
        // give the value in bits (plain value you want)
        // writing a value directly into a register is unsafe, in case another register needs it

        timer1.tcnt1.write(|w| unsafe { w.bits(0) });
        // warning that I don't use the result --> void_unwrap
        trig.set_high().void_unwrap();
        delay.delay_us(10u16);
        trig.set_low().void_unwrap();

        while echo.is_low().void_unwrap() {
            // if the timer is full exit this loop
            // if we don't receive an echo pulse
            if timer1.tcnt1.read().bits() >= 50000 {
                //reset to the beginning
                // if nothing is detected
                ufmt::uwriteln!(&mut serial, "Nothing was detected and jump to outer loop.\r").void_unwrap();
                continue 'outer;
            }
        }

        //restarting the timer
        timer1.tcnt1.write(|w| unsafe { w.bits(0) });

        // wait for the echo to get low again
        while echo.is_high().void_unwrap() {}

        // 1 timer count == 4 us
        // * 4 to get the value in microsecs
        // /58 to get the distance in cm
        // 1 count == 4 us, so we visualise that its 4us
        // we have a value in us
        let value = (timer1.tcnt1.read().bits() * 4) / 58;

        if value < 10 {
            loop {
                left_wheel_forward.set_high().void_unwrap();
                right_wheel_forward.set_high().void_unwrap();
                delay.delay_ms(2000 as u16);
                left_wheel_backward.set_high().void_unwrap();
                right_wheel_backward.set_high().void_unwrap();
                /*

                let duty_front = 191;
                pd3.set_duty(duty_front as u8);
                delay.delay_ms(2000u16);

                let duty_right = 0;
                pd3.set_duty(duty_front as u8);
                delay.delay_ms(2000u16);

                let duty_fleft = 128;
                pd3.set_duty(duty_front as u8);
                delay.delay_ms(2000u16);

                continue 'outer;
                */
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

/*
let mut serial = arduino_uno::Serial::new(
    // protocol to communicate bytes in 2 directions
    // USART0 is moved to serial, serial becomes the new owner
    //https://rahix.github.io/avr-hal/atmega328p_hal/usart/struct.Usart0.html
    dp.USART0,
    //
    // rx: receive pin (hardwired into the MCU)
    // tx : PD1 is the "hardcoded output"
    // the ownership is moved by writing explicitely
    // input, output is enforced at compile time,
    pins.d0,
    // d1 is converted  v---v into an output
    pins.d1.into_output(&mut pins.ddr),
    // choose well known baud rates (9600)
    57600,
);
 */
