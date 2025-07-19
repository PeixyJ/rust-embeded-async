use crate::button::ButtonDirection;
use crate::channel::Receiver;
use crate::timer::{Ticker, Timer};
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use fugit::ExtU64;
use microbit::gpio::NUM_COLS;
use nrf52833_hal::gpio::{Output, Pin, PushPull};
use nrf52833_hal::wdt::Active;
use rtt_target::rprintln;

enum LedState<'a> {
    Toggle,
    Wait(Timer<'a>),
}

pub struct LedTask<'a> {
    //传入当前列
    col: [Pin<Output<PushPull>>; NUM_COLS],
    //当前列索引
    active_col: usize,
    // 计时器
    ticker: &'a Ticker,
    // LED 状态
    state: LedState<'a>,
    // 接收按钮方向的通道
    receiver: Receiver<'a, ButtonDirection>,
}

impl<'a> LedTask<'a> {
    pub fn new(
        col: [Pin<Output<PushPull>>; NUM_COLS],
        ticker: &'a Ticker,
        receiver: Receiver<'a, ButtonDirection>,
    ) -> Self {
        Self {
            col,
            active_col: 0,
            ticker: &ticker,
            state: LedState::Toggle,
            receiver,
        }
    }

    fn shift(&mut self, direction: ButtonDirection) {
        rprintln!("Button pressed detected..");
        // 设置当前 LED 为高亮
        self.col[self.active_col].set_high().ok();
        // 根据按钮方向更新 active_col
        self.active_col = match direction {
            ButtonDirection::Left => match self.active_col {
                0 => 4,
                _ => self.active_col - 1,
            },
            ButtonDirection::Right => (self.active_col + 1) % NUM_COLS,
        };
        self.col[self.active_col].set_high().ok();
    }

    pub fn poll(&mut self) {
        match &mut self.state {
            //如果处于 Toggle 状态，切换 LED 状态
            LedState::Toggle => {
                rprintln!("Blinking LED {}", self.active_col);
                // 切换当前列的 LED 状态
                self.col[self.active_col].toggle().ok();
                // 设置下一个状态为等待状态
                self.state = LedState::Wait(Timer::new(1000.millis(), self.ticker));
            }
            LedState::Wait(timer) => {
                // 如果计时器到期，切换到 Toggle 状态
                if timer.is_ready() {
                    self.state = LedState::Toggle;
                }
                // 检查是否有按钮按下
                if let Some(direction) = self.receiver.receive() {
                    self.shift(direction);
                    self.state = LedState::Toggle;
                }
            }
        }
    }
}
