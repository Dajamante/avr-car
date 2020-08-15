// Macros to inform rust that the project will not use
// main and the standard library (lib std adds a layer to build the usual functions.)
#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;
use arduino_uno::prelude::*;

use crate::motors::{go_backward, go_forward, stop, turn_left, turn_right};
use crate::sensor::return_distance;
use arduino_uno::hal::port::mode::Floating;

mod motors;
mod sensor;


pub struct SensorUnit{
    trig: arduino_uno::hal::port::portb::PB4<arduino_uno::hal::port::mode::Output>,
    echo: arduino_uno::hal::port::portb::PB3<arduino_uno::hal::port::mode::Input<Floating>>,
    timer: arduino_uno::atmega328p::TC1,
}

// creates the main function
// attribute macro -> transforms the next as the entry point

// "!" is a never type. It informs nothing should return from the main function.
#[arduino_uno::entry]
fn main() -> ! {
    // we acquire first a singleton of all the peripherals (everything inside the MCU)
    // more information on raw registers abstraction here:
    // https://docs.rs/avr-device/0.2.1/avr_device/atmega328p/struct.Peripherals.html

    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut delay = arduino_uno::Delay::new();

    // all the ports are collapsed into the variable pins
    // docs on all pins: https://rahix.github.io/avr-hal/arduino_uno/struct.Pins.html
    // by default all pins are configured as Inputs and Floating
    // (pull up is to avoid undefined state. For arduino boards (5V), pull-up will allow up or down.
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // this is the console. To see the output do (on mac)
    // screen /dev/tty/<your_tty_here> 57600
    // ls /dev/tty* | grep usb --> get the usb connected
    // 57600 is the baud rate
    let mut serial = arduino_uno::Serial::new(
        // protocol to communicate bytes in 2 directions
        // USART0 is moved to serial, serial becomes the new owner
        // https://rahix.github.io/avr-hal/atmega328p_hal/usart/struct.Usart0.html
        dp.USART0,
        // the values below correspond to :
        // rx: receive pin (hardwired into the MCU)
        // tx : PD1 is the "hardcoded output"
        // the ownership is moved by writing explicitely input, output is enforced at compile time,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        // other well known baud rates are possible (9600)
        57600,
    );

    // making the timer available, with a prescaling
    let timer1 = dp.TC1;
    // initialisation :  we write over and set prescaler to 64
    // (1/(16e6/64)) * 2^16 (size of register) ~> takes 262 ms for a cycle
    timer1.tccr1b.write(|w| w.cs1().prescale_64());

    let timer2 = dp.TC2;
    // timer2 is used for pwm. pwm accept u8 values so the prescaling is chosen at 1024
    timer2.tccr2b.write(|w| w.cs2().prescale_1024());
    // setting the waveform generation mode bits WGM to 011 selects fast PWM.
    // Setting the COM2B bits to 10 provides non-inverted PWM for outputs A and B.
    // https://www.arduino.cc/en/Tutorial/SecretsOfArduinoPWM
    timer2.tccr2a.write(|w| w.wgm2().pwm_fast().com2b().match_clear());

    // We do not use pin 13, because it is also connected to an onboard LED marked "L"
    // ownership issues: we are moving the pins.d13 into first, the function into_output
    // second, into led. It needs the ddr register for configuration
    // (DDRx are used to configure the respective PORT as output/input)
    let trig = pins.d12.into_output(&mut pins.ddr);
    // floating input is set by default so we can configure echo without ddr
    let echo = pins.d11;

    // servo is best set as a struct for clarity, it will be send to
    // into a function in a module return distance
    let mut sensor_unit = SensorUnit{
        trig,
        echo,
        timer: timer1,
    };

    // pin d3 is hardwired with timer2.
    // so it will be a natural output. But the variable servo itself is never used.
    let mut _servo = pins.d3.into_output(&mut pins.ddr);

    // downgrading the pins allow to put them in an array and simplify functions:
    // according to docs : Downgrade this pin into a type that is generic over all pins.
    let left_forw = pins.d4.into_output(&mut pins.ddr).downgrade();
    let left_back = pins.d5.into_output(&mut pins.ddr).downgrade();
    let right_forw = pins.d6.into_output(&mut pins.ddr).downgrade();
    let right_back = pins.d7.into_output(&mut pins.ddr).downgrade();

    // we have now mutable wheels that can be sent to motor functions
    let mut wheels = [left_forw, left_back, right_forw, right_back];


    // the car is always going forward (and printing distance to console if connected to screen)
    // until it meets an obstacle.
    'outer: loop {
        timer2.ocr2b.write(|x| unsafe { x.bits(20) });
        go_forward(&mut wheels);

        let value = return_distance(&mut sensor_unit);
        ufmt::uwriteln!( & mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();

        if value < 10 {
            loop {
                stop(&mut wheels);

                timer2.ocr2b.write(|x| unsafe { x.bits(10) });
                let value_right = return_distance(&mut sensor_unit);
                ufmt::uwriteln!( & mut serial, "On right, we are {} cms away from target!\r", value).void_unwrap();

                delay.delay_ms(1000u16);

                timer2.ocr2b.write(|x| unsafe { x.bits(30) });
                let value_left = return_distance(&mut sensor_unit);
                ufmt::uwriteln!( & mut serial, "On left, we are {} cms away from target!\r", value).void_unwrap();

                delay.delay_ms(1000u16);

                if (value_left > value_right) && value_left > 20 {
                    turn_left(&mut wheels);

                } else if (value_right > value_left) && value_right > 20 {
                    turn_right(&mut wheels);

                } else {
                    go_backward(&mut wheels);
                    delay.delay_ms(1000u16);
                    turn_right(&mut wheels);

                }
                continue 'outer;
            }
        }
        // the sensor needs to wait approximately 60 ms between two waves.
        // we ensure that by waiting while the register reaches 25000
        // one count == 4 us, and 4us*0.000004 == 100 ms
        while sensor_unit.timer.tcnt1.read().bits() < 25000 {}

        // I honestly forgot why I print that twice...
        ufmt::uwriteln!( & mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
    }
}

