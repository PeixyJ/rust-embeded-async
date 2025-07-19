use nrf52833_hal::Rtc;
use nrf52833_pac::RTC0;

use fugit::{Duration, Instant};

type TickInstant = Instant<u64, 1, 32768>;
type TickDuration = Duration<u64, 1, 32768>;

pub struct Ticker {
    rtc: Rtc<RTC0>,
}

pub struct Timer<'a> {
    end_time: TickInstant,
    ticker: &'a Ticker,
}

impl<'a> Timer<'a> {
    pub fn new(duration: TickDuration, ticker: &'a Ticker) -> Self {
        Self {
            end_time: ticker.now() + duration,
            ticker,
        }
    }

    //检查定时器是否到期
    pub fn is_ready(&self) -> bool {
        self.ticker.now() >= self.end_time
    }
}
impl Ticker {
    pub fn new(rtc0: RTC0) -> Self {
        let rtc = Rtc::new(rtc0, 0).unwrap();
        //启动实时钟计数器
        rtc.enable_counter();
        Self { rtc }
    }

    //想要返回结果为毫秒 可以使用 fugit
    pub fn now(&self) -> TickInstant {
        //获取当前计数值
        TickInstant::from_ticks(self.rtc.get_counter() as u64)
    }
}
