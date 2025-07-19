use crate::channel::Sender;
use crate::timer::{Ticker, Timer};
use embedded_hal::digital::InputPin;
use fugit::ExtU64;
use nrf52833_hal::gpio::{Floating, Input, Pin};

#[derive(Clone, Copy)]
pub enum ButtonDirection {
    Left,
    Right,
}

enum ButtonState<'a> {
    WaitForPress,
    Debounce(Timer<'a>),
}

pub struct ButtonTask<'a> {
    pin: Pin<Input<Floating>>,
    ticker: &'a Ticker,
    direction: ButtonDirection,
    state: ButtonState<'a>,
    sender: Sender<'a, ButtonDirection>,
}

impl<'a> ButtonTask<'a> {
    pub fn new(
        pin: Pin<Input<Floating>>,
        ticker: &'a Ticker,
        direction: ButtonDirection,
        sender: Sender<'a, ButtonDirection>,
    ) -> Self {
        Self {
            pin,
            ticker,
            direction,
            state: ButtonState::WaitForPress,
            sender,
        }
    }

    pub fn poll(&mut self) {
        match &mut self.state {
            // 如果处于 WaitForPress 状态，检查按钮是否被按下
            ButtonState::WaitForPress => {
                if self.pin.is_low().unwrap() {
                    self.sender.send(self.direction);
                    // 按钮被按下，进入去抖动状态
                    self.state = ButtonState::Debounce(Timer::new(200.millis(), &self.ticker));
                }
            }
            // 如果处于 Debounce 状态，等待去抖动时间结束
            ButtonState::Debounce(timer) => {
                if timer.is_ready() && self.pin.is_low().unwrap() {
                    self.state = ButtonState::WaitForPress;
                }
            }
        }
    }
}
