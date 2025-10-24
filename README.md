rustino-stepseq
===============

A step sequencer for Arduino Nano written in Rust. Based on Look Mum No Computer's [8 STEP SEQUENCER](https://www.lookmumnocomputer.com/projects#/sequencer-keyboard).

> Note: I am still waiting on some components needed to build this circuit, so I cannot confirm
> whether or not it works yet.

Rust project for the _Arduino Nano New Bootloader_.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License

Currently, this project is being released without license as
I haven't yet discussed license specifics with Sam Battle from
Look Mum No Computer. Once I have discussed it with him, I will
update the license accordingly.
