rustino-stepseq
===============

A step sequencer for Arduino Nano written in Rust. Based on Look Mum No Computer's [8 STEP SEQUENCER](https://www.lookmumnocomputer.com/projects#/sequencer-keyboard).

## Update: 10/26/2025
As of writing, I have been able to test this with only 5 gate buttons and 8 step pots (as opposed to the full 16), but the source appears to be working.
The schematic likely still has some errors which I will soon iron out. I'll test the full circuit once I can find another breadboard.

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
