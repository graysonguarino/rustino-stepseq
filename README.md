rustino-stepseq
===============

A step sequencer for Arduino Nano written in Rust. Based on Look Mum No Computer's 
[8 STEP SEQUENCER](https://www.lookmumnocomputer.com/projects#/sequencer-keyboard).

[A demo video of a breadboard of the circuit can be found here](https://www.youtube.com/watch?v=a9vYyX_fQB8).

Rust project for the _Arduino Nano New Bootloader_.

## Update: 10/31/2025 (Happy Halloween! 👻)
This circuit and source code appears to be working! I have now been able to fully
breadboard the circuit and, bar the max 4.6V CV outs (documented in [#2](https://github.com/graysonguarino/rustino-stepseq/issues/2)), 
the step sequencer is working. I cannot confirm 1:1 behavior matching with LOOK MUM
NO COMPUTER's as this uses internal pullups where possible so the logic is slightly 
different.

If you build the circuit from the schematic and something is wrong, please 
view the images and videos in `breadboard/` to see a known-working implementation and 
leave an issue telling me what you found.

## Potential future improvements

* Fix [#2](https://github.com/graysonguarino/rustino-stepseq/issues/2) with a dual opamp.
* Have someone with more electronics experience than me evaluate the schematic.
   * **If you've got an EE background, please contribute if I messed up!**

I think that, as long as this circuit functions, it will be good enough for my needs and I
will not be developing this any further. I have ideas for a more featured step sequencer though,
so once I get the necessary parts in, keep an eye out for that. 👀

## Differences from LOOK MUM NO COMPUTER's

* Internal pullup resistors were used whenever possible, which reduces the BOM.
   * **This means the circuit differs from LMNC's, so don't follow that wiring diagram**
   **unless you change the logic in `peripherals.rs`!**

## Build Notes

* Arduino D12 and D13 require pulldown resistors as D13 is used to light the onboard LED.
   * If you are using buttons instead of an SPDT switch for FWD/BWD, D12 does not need a pulldown
   resistor, but you will need to wire the D12 button to ground instead of +5V and 
   modify `forwards_clock_input` in `Peripherals::new()` to match the settings of 
   `reset_input` and `zero_input`. Since an SPDT switch is used in the schematic, both 
   D12 and D13 will be connected to +5V when the switch is toggled, so the code is set to reflect that.
* Pullup resistors must be wired for Arduino A6 and A7 as these are not GPIO but ADC inputs and
thus do not have internal pullup resistors.
* The diodes on `FWD_CLK_IN` and `BWD_CLK_IN` don't appear to be necessary if you're sure you're
only ever providing a 0->+5V input. Since Eurorack allows for +/-5V CV, the circuit may act funky
if you accidentally feed it a negative voltage without these diodes.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.
   * Tip: I have assumed your Arduino is on `/dev/tty.usbserial-210`, but **it likely won't be.** 
   If you're on a *nix system, try running `ls /dev/tty.usbserial*` and if you don't have other 
   devices plugged in besides the Arduino, it's likely that one. Change the `port` entry in 
   `Ravedude.toml` to that value.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License

Currently, this project is being released without license as
I haven't yet discussed license specifics with Sam Battle from
Look Mum No Computer. Once I have discussed it with him, I will
update the license accordingly.

Here is the unofficial license he provided in the source code:

```
//MORE INFO CHECK LOOKMUMNOCOMPUTER.COM
//PLEASE EMAIL ME COMPUTER@LOOKMUMNOCOMPUTER.COM OR LOOKMUMNOCOMPUTER@GMAIL.COM IF ANY QUESTIONS
//ANY MODIFICATIONS PLEASE LET ME KNOW AND I CAN FEATURE ON THE WEBSITE
//OPEN SOURCE. DONT USE IN A PRODUCT OR WHATEVER WITHOUT TALKING TO ME
//SAM BATTLE 2017
//BOOM
```

Boom indeed, Sam. Thanks again for all you do!
