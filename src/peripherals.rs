use crate::{ADC_HIGH_THRESHOLD, NUM_BUTTON_INPUTS, NUM_GATE_OUTPUTS};
use arduino_hal::{
    Adc,
    hal::{self, port::Dynamic},
    pins,
    port::{
        Pin,
        mode::{Input, Output},
    },
};

pub struct Peripherals {
    adc: Adc,
    pub button_inputs: [InputPin; NUM_BUTTON_INPUTS],
    pub gate_outputs: [OutputPin; NUM_GATE_OUTPUTS],
    reset_input: InputPin,
    zero_input: InputPin,
    forwards_clock_input: InputPin,
    backwards_clock_input: InputPin,
}

pub enum InputSelector {
    Button(usize),
    Reset,
    Zero,
    ForwardsClock,
    BackwardsClock,
}

impl Peripherals {
    pub fn new(dp: arduino_hal::Peripherals) -> Self {
        let pins = pins!(dp);
        Self {
            adc: Adc::new(dp.ADC, Default::default()),
            button_inputs: [
                InputPin::GpioPin {
                    pin: pins.a0.into_pull_up_input().forget_imode().downgrade(),
                    on_high: true,
                },
                InputPin::GpioPin {
                    pin: pins.a1.into_pull_up_input().forget_imode().downgrade(),
                    on_high: true,
                },
                InputPin::GpioPin {
                    pin: pins.a2.into_pull_up_input().forget_imode().downgrade(),
                    on_high: true,
                },
                InputPin::GpioPin {
                    pin: pins.a3.into_pull_up_input().forget_imode().downgrade(),
                    on_high: true,
                },
                InputPin::GpioPin {
                    pin: pins.a4.into_pull_up_input().forget_imode().downgrade(),
                    on_high: true,
                },
                InputPin::GpioPin {
                    pin: pins.a5.into_pull_up_input().forget_imode().downgrade(),
                    on_high: true,
                },
                InputPin::AdcPin {
                    pin: hal::adc::channel::ADC6.into_channel(),
                    on_high: true,
                },
                InputPin::AdcPin {
                    pin: hal::adc::channel::ADC7.into_channel(),
                    on_high: true,
                },
            ],
            gate_outputs: [
                OutputPin::OnHigh(pins.d2.into_output_high().downgrade()),
                OutputPin::OnHigh(pins.d3.into_output().downgrade()),
                OutputPin::OnHigh(pins.d4.into_output().downgrade()),
                OutputPin::OnHigh(pins.d5.into_output().downgrade()),
                OutputPin::OnHigh(pins.d6.into_output().downgrade()),
                OutputPin::OnHigh(pins.d7.into_output().downgrade()),
                OutputPin::OnHigh(pins.d8.into_output().downgrade()),
                OutputPin::OnHigh(pins.d9.into_output().downgrade()),
            ],
            reset_input: InputPin::GpioPin {
                pin: pins.d10.into_pull_up_input().forget_imode().downgrade(),
                on_high: false,
            },
            zero_input: InputPin::GpioPin {
                pin: pins.d11.into_pull_up_input().forget_imode().downgrade(),
                on_high: false,
            },
            forwards_clock_input: InputPin::GpioPin {
                pin: pins.d12.into_floating_input().forget_imode().downgrade(),
                on_high: true,
            },
            backwards_clock_input: InputPin::GpioPin {
                pin: pins.d13.into_floating_input().forget_imode().downgrade(),
                on_high: true,
            },
        }
    }

    pub fn is_on(&mut self, selector: InputSelector) -> bool {
        match selector {
            InputSelector::Button(idx) => self.button_inputs[idx].is_on(&mut self.adc),
            InputSelector::Reset => self.reset_input.is_on(&mut self.adc),
            InputSelector::Zero => self.zero_input.is_on(&mut self.adc),
            InputSelector::ForwardsClock => self.forwards_clock_input.is_on(&mut self.adc),
            InputSelector::BackwardsClock => self.backwards_clock_input.is_on(&mut self.adc),
        }
    }
}

pub enum InputPin {
    GpioPin {
        pin: hal::port::Pin<Input, Dynamic>,
        on_high: bool,
    },
    AdcPin {
        pin: hal::adc::Channel,
        on_high: bool,
    },
}

impl InputPin {
    fn is_on(&self, adc: &mut Adc) -> bool {
        match self {
            InputPin::GpioPin { pin, on_high } => {
                let high = pin.is_high();
                if *on_high { high } else { !high }
            }
            InputPin::AdcPin { pin, on_high } => {
                let high = adc.read_blocking(pin) >= ADC_HIGH_THRESHOLD;
                if *on_high { high } else { !high }
            }
        }
    }
}

pub enum OutputPin {
    OnHigh(Pin<Output>),
    OnLow(Pin<Output>),
}

impl OutputPin {
    pub fn set_on(&mut self) {
        match self {
            OutputPin::OnHigh(pin) => pin.set_high(),
            OutputPin::OnLow(pin) => pin.set_low(),
        }
    }

    pub fn set_off(&mut self) {
        match self {
            OutputPin::OnHigh(pin) => pin.set_low(),
            OutputPin::OnLow(pin) => pin.set_high(),
        }
    }
}
