## Arduino Uno and Rust Experiment - Controlled Blink

This is my Rust solution for the second experiment through
Vilros Ultimate Starter Kit Guide. This uses the same circuit
as Circuit #2 utilizing the potentiometer.

### Schematic

TODO

### Building and Executing the Program

In order to run this project, you must be using Rust's Nightly version.
This should automatically be recognized if you're using rustup due to
`rust-toolchain` file. In addition you'll want a few avr third-party tools
which you can look at the [`avr-rust` book for how to install](https://book.avr-rust.com/002.1-installing-required-third-party-tools.html).

From there, you can run `cargo build` or `cargo build --release`. This will
build out either a `debug` or `release` under your `target/avr-atmega328p`:

```shell
target/avr-atmega328p/debug/arduino-controlled-blink.elf

# or

target/avr-atmega328p/release/arduino-controlled-blink.elf
```

Once successfully built, you can then flash (another way of saying upload)
your program into the arduino uno using the addition file provided, `uno-runner.sh`.

Make sure to make the file executable by `chmod +x uno-runner.sh`. This will help
set up the appropriate flags for using `avrdude`. You'll also want to check the file
to set the appropriate SERIAL_PORT flag, for instance mine was `/dev/tty.usbmodem144101`.

To flash the arduino uno, make sure it is plugged in and then run the command:

```shell
# if you ran cargo build
./uno-runner.sh target/avr-atmega328p/debug/arduino-controlled-blink.elf

# or if you ran cargo build --release
./uno-runner.sh target/avr-atmega328p/release/arduino-controlled-blink.elf
```