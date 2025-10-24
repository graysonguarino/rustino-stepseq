#![no_std]
#![no_main]

mod peripherals;
mod state;

use crate::{peripherals::Peripherals, state::State};
use panic_halt as _;

pub const ADC_HIGH_THRESHOLD: u16 = 1000;
pub const NUM_STEPS: usize = 8;
pub const NUM_BUTTON_INPUTS: usize = NUM_STEPS;
pub const NUM_GATE_OUTPUTS: usize = NUM_STEPS;
pub const MAX_STEP: usize = NUM_STEPS - 1;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let peripherals = Peripherals::new(dp);
    let mut state = State::new(peripherals);

    loop {
        state.update();
    }
}
