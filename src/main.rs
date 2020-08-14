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

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    let timer1 = dp.TC1; //make the timer avaible
    timer1.tccr1b.write(|w| w.cs1().prescale_64());
    // Waveform Generation Mode bits (WGM): these control the overall mode of the timer.
    // (These bits are split between TCCRnA and TCCRnB.)
    // Clock Select bits (CS): these control the clock prescaler

    let mut timer2 = dp.TC2;

    //16e6/128 == 62500
    //https://sites.google.com/site/qeewiki/books/avr-guide/pwm-on-the-atmega328

    // same as  cs2().bits(0b100),
    timer2.tccr2b.write(|w| w.cs2().prescale_1024());
    // Both fast PWM and phase correct PWM have an additional mode that gives control over the output frequency.
    // In this mode, the timer counts from 0 to OCRA (the value of output compare register A),
    //16000000 Hz (clock speed)/1024 (prescale)= 15625 Hz
    //15625 Hz /50 Hz (required servo frequenzy)= 312 (OCR1A top limit 4999)
    /*
    then you need to select Fast PWM mode for the WGM (Waveform Generation Mode)
    and then you need to set the COM (Compare Output Mode)
    for your output (either COM2A or COM2B) to MATCH_CLEAR

    you call .fast_pwm() on that handle to write the bits
     to the value representing FAST_PWM mode. as you're now done with the WGM2 field,
     you get back the w handle representing the whole register
     */
    timer2.tccr2a.write(|w| w.wgm2().pwm_fast().com2b().match_clear());

    //  Setting the COM2A bits and COM2B bits to 10 provides non-inverted PWM for outputs A and B. needed?


    let mut trig = pins.d12.into_output(&mut pins.ddr);
    // floating input by default
    let echo = pins.d11;
    let mut servo = pins.d3.into_output(&mut pins.ddr);

    'outer: loop {
        timer1.tcnt1.write(|w| unsafe { w.bits(0) });
        // warning that I don't use the result --> void_unwrap
        trig.set_high().void_unwrap();
        delay.delay_us(10u16);
        trig.set_low().void_unwrap();

        while echo.is_low().void_unwrap() {
            // more than 200 ms ( = 50000)
            if timer1.tcnt1.read().bits() >= 65000 {
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
            //((16e6 clock/1024 preschaling)/50 hz as required by servo) * 1.5ms/20ms
            timer2.ocr2b.write(|x| unsafe { x.bits(23) });
            delay.delay_ms(1000u16);

        }

        while timer1.tcnt1.read().bits() < 25000 {}

        ufmt::uwriteln!(&mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
    }
}
