//! ## Classical obstacle avoiding robot with Rust
//! This project takes and Arduino classic and tries to port it to
//! Rust as a beginner project.
//! It uses avr-hal crate for abstraction.
//! For hardware it uses a simple HC-SR04 sensor, an L298N motor driver and a SG90
//! servo motor (details in the Readme).
//! The sensor is reading distance every > 100ms and the robot should take appropriate
//! action if an obstacle is detected.

// Macros to inform rust that the project will not use
// main and the standard library (lib std adds a layer to build the usual functions.)
#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::hal::port::mode::Floating;
use arduino_uno::prelude::*;
use arduino_uno::pwm;

use crate::motors::{go_backward, go_forward, stop, turn_left, turn_right};
use crate::sensor::{return_distance, SensorUnit};
use crate::servo::ServoUnit;

mod motors;
mod sensor;
mod servo;

const WAIT_BETWEEN_ACTIONS: u16 = 1000u16;
const MINIMAL_DISTANCE: u16 = 10u16;
const ACCEPTABLE_DISTANCE: u16 = 10u16;

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

    // initialisation of timer 1 :  we write over and set prescaler to 64
    // (1/(16e6/64)) * 2^16 (size of register) ~> takes 262 ms for a cycle
    // timer1 is shared with the sensor unit
    let timer1 = dp.TC1;
    timer1.tccr1b.write(|w| w.cs1().prescale_64());

    let mut timer2 = pwm::Timer2Pwm::new(dp.TC2, pwm::Prescaler::Prescale1024);

    let mut pd3 = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer2);
    pd3.enable();
    let mut servo_unit = ServoUnit {
        servo: pd3,
    };

    // servo is best set as a struct for clarity, it will be send to

    let mut sensor_unit = SensorUnit {
        // We do not use pin 13, because it is also connected to an onboard LED marked "L"
        // ownership issues: we are moving the pins.d13 into first, the function into_output
        // second, into led. It needs the ddr register for configuration
        // (DDRx are used to configure the respective PORT as output/input)
        trig: pins.d12.into_output(&mut pins.ddr),
        // floating input is set by default so we can configure echo without ddr
        echo: pins.d11,
        timer: timer1,
    };

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
        servo_unit.look_front();
        go_forward(&mut wheels);

        if let Ok(value) = return_distance(&mut sensor_unit) {
            ufmt::uwriteln!( & mut serial, "I am here");
            ufmt::uwriteln!( & mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
            let mut right:u16;
            let mut left:u16;
            if value < MINIMAL_DISTANCE {

                'look_right_left: loop {
                    stop(&mut wheels);

                    servo_unit.look_right();
                    if let Ok(value_right) = return_distance(&mut sensor_unit){
                        ufmt::uwriteln!( & mut serial, "I am right");
                        ufmt::uwriteln!( & mut serial, "On right, we are {} cms away from target!\r", value_right).void_unwrap();
                        right = value_right;
                    } else { continue 'look_right_left}

                    delay.delay_ms(WAIT_BETWEEN_ACTIONS);

                    servo_unit.look_left();
                    if let Ok(value_left) = return_distance(&mut sensor_unit){
                        ufmt::uwriteln!( & mut serial, "I am left");
                        ufmt::uwriteln!( & mut serial, "On left, we are {} cms away from target!\r", value_left).void_unwrap();
                        left = value_left;
                    }else{
                        continue 'look_right_left
                    }

                    delay.delay_ms(WAIT_BETWEEN_ACTIONS);

                    if (left > right) && left > ACCEPTABLE_DISTANCE {
                        turn_left(&mut wheels);
                    } else if (right > left) && right > ACCEPTABLE_DISTANCE {
                        turn_right(&mut wheels);
                    } else {
                        go_backward(&mut wheels);
                        delay.delay_ms(WAIT_BETWEEN_ACTIONS);
                        turn_right(&mut wheels);
                    }

                }
            }
            // the sensor needs to wait approximately 60 ms between two waves.
            // we ensure that by waiting while the register reaches 25000
            // one count == 4 us, and 4us*0.000004 == 100 ms
            while sensor_unit.timer.tcnt1.read().bits() < 25000 {}

            ufmt::uwriteln!( & mut serial, "I am on botom");
        }else{
            continue 'outer;
        }
    }
}

