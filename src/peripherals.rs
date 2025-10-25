use crate::{ADC_HIGH_THRESHOLD, NUM_BUTTON_INPUTS, NUM_GATE_OUTPUTS};
use arduino_hal::{
    Adc,
    hal::{self, port::Dynamic},
    pins,
    port::{
        Pin,
        mode::{Input, PullUp},
    },
};

pub struct Peripherals {
    adc: Adc,
    pub button_inputs: [InputPinType; NUM_BUTTON_INPUTS],
    pub gate_outputs: [OutputPinType; NUM_GATE_OUTPUTS],
    reset_input: PullupPin,
    zero_input: PullupPin,
    forwards_clock_input: PullupPin,
    backwards_clock_input: PullupPin,
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
                InputPinType::PullupPin(PullupPin {
                    pin: pins.a0.into_pull_up_input().downgrade(),
                }),
                InputPinType::PullupPin(PullupPin {
                    pin: pins.a1.into_pull_up_input().downgrade(),
                }),
                InputPinType::PullupPin(PullupPin {
                    pin: pins.a2.into_pull_up_input().downgrade(),
                }),
                InputPinType::PullupPin(PullupPin {
                    pin: pins.a3.into_pull_up_input().downgrade(),
                }),
                InputPinType::PullupPin(PullupPin {
                    pin: pins.a4.into_pull_up_input().downgrade(),
                }),
                InputPinType::PullupPin(PullupPin {
                    pin: pins.a5.into_pull_up_input().downgrade(),
                }),
                InputPinType::AdcPin(AdcPin {
                    pin: hal::adc::channel::ADC6.into_channel(),
                }),
                InputPinType::AdcPin(AdcPin {
                    pin: hal::adc::channel::ADC7.into_channel(),
                }),
            ],
            gate_outputs: [
                OutputPinType::Output(pins.d2.into_output().downgrade()),
                OutputPinType::PullUp(pins.d3.into_pull_up_input().downgrade()),
                OutputPinType::PullUp(pins.d4.into_pull_up_input().downgrade()),
                OutputPinType::PullUp(pins.d5.into_pull_up_input().downgrade()),
                OutputPinType::PullUp(pins.d6.into_pull_up_input().downgrade()),
                OutputPinType::PullUp(pins.d7.into_pull_up_input().downgrade()),
                OutputPinType::PullUp(pins.d8.into_pull_up_input().downgrade()),
                OutputPinType::PullUp(pins.d9.into_pull_up_input().downgrade()),
            ],
            reset_input: PullupPin {
                pin: pins.d10.into_pull_up_input().downgrade(),
            },
            zero_input: PullupPin {
                pin: pins.d11.into_pull_up_input().downgrade(),
            },
            forwards_clock_input: PullupPin {
                pin: pins.d12.into_pull_up_input().downgrade(),
            },
            backwards_clock_input: PullupPin {
                pin: pins.d13.into_pull_up_input().downgrade(),
            },
        }
    }

    pub fn is_high(&mut self, selector: InputSelector) -> bool {
        match selector {
            InputSelector::Button(idx) => self.button_inputs[idx].is_high(&mut self.adc),
            InputSelector::Reset => self.reset_input.is_high(&mut self.adc),
            InputSelector::Zero => self.zero_input.is_high(&mut self.adc),
            InputSelector::ForwardsClock => self.forwards_clock_input.is_high(&mut self.adc),
            InputSelector::BackwardsClock => self.backwards_clock_input.is_high(&mut self.adc),
        }
    }
}

pub enum InputPinType {
    PullupPin(PullupPin),
    AdcPin(AdcPin),
}

pub trait InputPin: Sized {
    // Returns true if input is HIGH, false if LOW.
    fn is_high(&self, adc: &mut Adc) -> bool;
}

impl InputPin for InputPinType {
    fn is_high(&self, adc: &mut Adc) -> bool {
        match self {
            InputPinType::PullupPin(pin) => pin.is_high(adc),
            InputPinType::AdcPin(pin) => pin.is_high(adc),
        }
    }
}

pub struct PullupPin {
    pin: hal::port::Pin<Input<PullUp>, Dynamic>,
}

impl InputPin for PullupPin {
    fn is_high(&self, _: &mut Adc) -> bool {
        self.pin.is_high()
    }
}

pub struct AdcPin {
    pin: hal::adc::Channel,
}

impl InputPin for AdcPin {
    fn is_high(&self, adc: &mut Adc) -> bool {
        adc.read_blocking(&self.pin) >= ADC_HIGH_THRESHOLD
    }
}

// The idea is this:
// 1. Button inputs are always set to INPUT_PULLUP
// 1a. Since, on the Nano, pins A5 and A6 are not digital,
// they do not have an INPUT_PULLUP, so I'll need to wire a
// pullup resistor myself for those.
// 2. Gate outputs are set to outputs if they're active, but
// set to INPUT_PULLUP when they are out of use

// Wrapper that allows safe in-place mutation of pin modes
pub enum OutputPinType {
    Output(Pin<hal::port::mode::Output>),
    PullUp(Pin<hal::port::mode::Input<PullUp>>),
    // Temporary state used during transitions
    Transitioning,
}

impl OutputPinType {
    #[allow(clippy::wrong_self_convention)]
    pub fn into_output(&mut self) {
        let old = core::mem::replace(self, OutputPinType::Transitioning);
        *self = match old {
            OutputPinType::Output(pin) => OutputPinType::Output(pin),
            OutputPinType::PullUp(pin) => OutputPinType::Output(pin.into_output()),
            OutputPinType::Transitioning => panic!("Invalid state"),
        };

        // LOW is on, and we always want the single output to be on.
        if let OutputPinType::Output(pin) = self {
            pin.set_low();
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_pullup(&mut self) {
        let old = core::mem::replace(self, OutputPinType::Transitioning);
        *self = match old {
            OutputPinType::Output(pin) => OutputPinType::PullUp(pin.into_pull_up_input()),
            OutputPinType::PullUp(pin) => OutputPinType::PullUp(pin),
            OutputPinType::Transitioning => panic!("Invalid state"),
        };
    }
}
