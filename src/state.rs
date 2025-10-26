use crate::{
    MAX_STEP,
    peripherals::{InputSelector, Peripherals},
};

pub struct State {
    peripherals: Peripherals,
    last_step: Step,
    last_reset_on: bool,
    last_zero_on: bool,
    last_forwards_clock_on: bool,
    last_backwards_clock_on: bool,
}

impl State {
    fn turned_on(&self, current: bool, last: bool) -> bool {
        current && current != last
    }

    fn turned_off(&self, current: bool, last: bool) -> bool {
        !current && current != last
    }

    pub fn new(mut peripherals: Peripherals) -> Self {
        let last_reset_on = peripherals.is_on(InputSelector::Reset);
        let last_zero_on = peripherals.is_on(InputSelector::Zero);
        let last_forwards_clock_on = peripherals.is_on(InputSelector::ForwardsClock);
        let last_backwards_clock_on = peripherals.is_on(InputSelector::BackwardsClock);

        Self {
            peripherals,
            last_step: Step { step: Some(0) },
            last_reset_on,
            last_zero_on,
            last_forwards_clock_on,
            last_backwards_clock_on,
        }
    }

    pub fn update(&mut self) {
        let reset_on = self.peripherals.is_on(InputSelector::Reset);
        let zero_on = self.peripherals.is_on(InputSelector::Zero);
        let forwards_clock_on = self.peripherals.is_on(InputSelector::ForwardsClock);
        let backwards_clock_on = self.peripherals.is_on(InputSelector::BackwardsClock);
        let mut current_step = self.last_step;

        if self.turned_off(forwards_clock_on, self.last_forwards_clock_on) {
            current_step.increment();
        }
        self.last_forwards_clock_on = forwards_clock_on;

        if self.turned_off(backwards_clock_on, self.last_backwards_clock_on) {
            current_step.decrement();
        }
        self.last_backwards_clock_on = backwards_clock_on;

        if self.turned_on(zero_on, self.last_zero_on) {
            current_step.zero();
        }
        self.last_zero_on = zero_on;

        if self.turned_on(reset_on, self.last_reset_on) {
            current_step.reset();
        }
        self.last_reset_on = reset_on;

        // If button[idx] is pressed, force playback of that step.
        // Highest idx pressed gets priority.
        for idx in 0..self.peripherals.button_inputs.len() {
            if !self.peripherals.is_on(InputSelector::Button(idx)) {
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
                .for_each(|gate| gate.set_off());
            return;
        }

        // Disable gate output for last_step if one existed
        if let Some(last_step) = self.last_step.get() {
            self.peripherals.gate_outputs[last_step].set_off();
        }

        // Enable gate output for current_step if one exists
        if let Some(current_step) = current_step.get() {
            self.peripherals.gate_outputs[current_step].set_on();
        }
    }
}

#[derive(Clone, Copy)]
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
        if let Some(step) = self.step {
            if step == 0 {
                Some(MAX_STEP)
            } else {
                Some(step - 1)
            }
        } else {
            Some(MAX_STEP)
        }
    }

    pub fn get_next(&self) -> Option<usize> {
        if let Some(step) = self.step {
            if step == MAX_STEP {
                Some(0)
            } else {
                Some(step + 1)
            }
        } else {
            Some(0)
        }
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
