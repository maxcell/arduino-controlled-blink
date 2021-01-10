#![no_std]
#![no_main]

use arduino_uno::adc;
use arduino_uno::prelude::*;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    // Grab possible peripherals
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    // Initialize an instance of the arduino uno with all pins
    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    // Helpful for reading the signals being sent
    // to A0
    // let mut serial = arduino_uno::Serial::new(
    //     peripherals.USART0,
    //     pins.d0,
    //     pins.d1.into_output(&mut pins.ddr),
    //     9600.into_baudrate(),
    // );

    let mut adc = adc::Adc::new(peripherals.ADC, Default::default());

    // Send a digital output signal (this is the LED)
    // connects to digital pin 13
    let mut led = pins.d13.into_output(&mut pins.ddr);

    // Set up analog input (this is the potentiometer)
    // connects to analog pin 0
    let mut potentiometer = pins.a0.into_analog_input(&mut adc);

    loop {
        // Read the current signal from the potentiometer
        let signal: u16 = nb::block!(adc.read(&mut potentiometer)).void_unwrap();

        // Turn on the LED
        led.toggle().void_unwrap();

        // Wait for however long the current signal is
        arduino_uno::delay_ms(signal);

        // Turn off
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(signal);
    }
}
