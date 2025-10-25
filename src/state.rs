use crate::{
    MAX_STEP,
    peripherals::{InputSelector, Peripherals},
};

pub struct State {
    peripherals: Peripherals,
    last_step: Step,
    last_reset_high: bool,
    last_zero_high: bool,
    last_forwards_clock_high: bool,
    last_backwards_clock_high: bool,
}

impl State {
    fn falling_edge(&self, current: bool, last: bool) -> bool {
        !current && current != last
    }

    pub fn new(mut peripherals: Peripherals) -> Self {
        let last_reset_high = peripherals.is_high(InputSelector::Reset);
        let last_zero_high = peripherals.is_high(InputSelector::Zero);
        let last_forwards_clock_high = peripherals.is_high(InputSelector::ForwardsClock);
        let last_backwards_clock_high = peripherals.is_high(InputSelector::BackwardsClock);

        Self {
            peripherals,
            last_step: Step::default(),
            last_reset_high,
            last_zero_high,
            last_forwards_clock_high,
            last_backwards_clock_high,
        }
    }

    // Remember: LOW means "on", HIGH means "off"
    pub fn update(&mut self) {
        let reset_high = self.peripherals.is_high(InputSelector::Reset);
        let zero_high = self.peripherals.is_high(InputSelector::Zero);
        let forwards_clock_high = self.peripherals.is_high(InputSelector::ForwardsClock);
        let backwards_clock_high = self.peripherals.is_high(InputSelector::BackwardsClock);
        let mut current_step = self.last_step;

        if self.falling_edge(forwards_clock_high, self.last_forwards_clock_high) {
            current_step.increment();
        }
        self.last_forwards_clock_high = forwards_clock_high;

        if self.falling_edge(backwards_clock_high, self.last_backwards_clock_high) {
            current_step.decrement();
        }
        self.last_backwards_clock_high = backwards_clock_high;

        if zero_high != self.last_zero_high {
            if !zero_high {
                current_step.zero();
            } else {
                current_step.reset();
            }
        }
        self.last_zero_high = zero_high;

        if self.falling_edge(reset_high, self.last_reset_high) {
            current_step.reset();
        }
        self.last_reset_high = reset_high;

        // If button[idx] is pressed, force playback of that step.
        // Highest idx pressed gets priority.
        for idx in 0..self.peripherals.gate_outputs.len() {
            if !self.peripherals.is_high(InputSelector::Button(idx)) {
                current_step.set(Some(idx));
            }
        }

        self.update_outputs_for_current_step(&current_step);
        self.last_step = current_step;
    }

    fn update_outputs_for_current_step(&mut self, current_step: &Step) {
        // If current_step is None, ensure that all gate outs are disabled
        if current_step.get().is_none() {
            self.peripherals
                .gate_outputs
                .iter_mut()
                .for_each(|gate| gate.into_pullup());
            return;
        }

        // Disable gate output for last_step if one existed
        if let Some(last_step) = self.last_step.get() {
            self.peripherals.gate_outputs[last_step].into_pullup();
        }

        // Enable gate output for current_step if one exists
        if let Some(current_step) = current_step.get() {
            self.peripherals.gate_outputs[current_step].into_output();
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Step {
    step: Option<usize>,
}

impl Step {
    pub fn get(&self) -> Option<usize> {
        self.step
    }

    pub fn set(&mut self, new_step: Option<usize>) {
        self.step = new_step;
    }

    pub fn get_previous(&self) -> Option<usize> {
        self.step
            .map(|step| if step == 0 { MAX_STEP } else { step - 1 })
    }

    pub fn get_next(&self) -> Option<usize> {
        self.step
            .map(|step| if step == MAX_STEP { 0 } else { step + 1 })
    }

    pub fn increment(&mut self) {
        self.step = self.get_next();
    }

    pub fn decrement(&mut self) {
        self.step = self.get_previous();
    }

    pub fn zero(&mut self) {
        self.step = None;
    }

    pub fn reset(&mut self) {
        self.step = Some(0);
    }
}
