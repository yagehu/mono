//! A delay driver based on SysTick.

use crate::peripheral::{syst::SystClkSource, SYST};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// System timer (SysTick) as a delay provider.
pub struct Delay {
    syst: SYST,
    ahb_frequency: u32,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider.
    ///
    /// `ahb_frequency` is a frequency of the AHB bus in Hz.
    #[inline]
    pub fn new(mut syst: SYST, ahb_frequency: u32) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay {
            syst,
            ahb_frequency,
        }
    }

    /// Releases the system timer (SysTick) resource.
    #[inline]
    pub fn free(self) -> SYST {
        self.syst
    }

    fn _delay_us(&mut self, us: u32) {
        let ticks = (us as u64) * (self.ahb_frequency as u64) / 1_000_000;

        let full_cycles = ticks >> 24;
        if full_cycles > 0 {
            self.syst.set_reload(0xffffff);
            self.syst.clear_current();
            self.syst.enable_counter();

            for _ in 0..full_cycles {
                while !self.syst.has_wrapped() {}
            }
        }

        let ticks = (ticks & 0xffffff) as u32;
        if ticks > 1 {
            self.syst.set_reload(ticks - 1);
            self.syst.clear_current();
            self.syst.enable_counter();

            while !self.syst.has_wrapped() {}
        }

        self.syst.disable_counter();
    }
}

impl DelayMs<u32> for Delay {
    #[inline]
    fn delay_ms(&mut self, mut ms: u32) {
        // 4294967 is the highest u32 value which you can multiply by 1000 without overflow
        while ms > 4294967 {
            self.delay_us(4294967000u32);
            ms -= 4294967;
        }
        self.delay_us(ms * 1_000);
    }
}

// This is a workaround to allow `delay_ms(42)` construction without specifying a type.
impl DelayMs<i32> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: i32) {
        assert!(ms >= 0);
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u16> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayMs<u8> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayUs<u32> for Delay {
    #[inline]
    fn delay_us(&mut self, us: u32) {
        self._delay_us(us);
    }
}

// This is a workaround to allow `delay_us(42)` construction without specifying a type.
impl DelayUs<i32> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: i32) {
        assert!(us >= 0);
        self.delay_us(us as u32);
    }
}

impl DelayUs<u16> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32::from(us))
    }
}

impl DelayUs<u8> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32::from(us))
    }
}
