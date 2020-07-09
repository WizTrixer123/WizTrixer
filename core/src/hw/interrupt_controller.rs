use bitflags::*;
use crate::hw::{mmu::IORegister, Scheduler};

pub struct InterruptController {
    pub enable: InterruptEnable,
    pub master_enable: InterruptMasterEnable,
    pub request: InterruptRequest,
}

impl InterruptController {
    pub fn new() -> InterruptController {
        InterruptController {
            enable: InterruptEnable::empty(),
            master_enable: InterruptMasterEnable::empty(),
            request: InterruptRequest::empty(),
        }
    }

    pub fn interrupts_requested(&self) -> bool {
        self.master_enable.bits() != 0 && (self.request.bits() & self.enable.bits()) != 0
    }
}

bitflags! {
    pub struct InterruptEnable: u32 {
        const VBLANK = 1 << 0;
        const HBLANK = 1 << 1;
        const VCOUNTER_MATCH = 1 << 2;
        const TIMER0_OVERFLOW = 1 << 3;
        const TIMER1_OVERFLOW = 1 << 4;
        const TIMER2_OVERFLOW = 1 << 5;
        const TIMER3_OVERFLOW = 1 << 6;
        const SERIAL = 1 << 7;
        const DMA0 = 1 << 8;
        const DMA1 = 1 << 9;
        const DMA2 = 1 << 10;
        const DMA3 = 1 << 11;
        const KEYPAD = 1 << 12;
        const GAME_PAK = 1 << 13;
    }
}

bitflags! {
    pub struct InterruptMasterEnable: u32 {
        const ENABLE = 1 << 0;
    }
}

bitflags! {
    pub struct InterruptRequest: u32 {
        const VBLANK = 1 << 0;
        const HBLANK = 1 << 1;
        const VCOUNTER_MATCH = 1 << 2;
        const TIMER0_OVERFLOW = 1 << 3;
        const TIMER1_OVERFLOW = 1 << 4;
        const TIMER2_OVERFLOW = 1 << 5;
        const TIMER3_OVERFLOW = 1 << 6;
        const SERIAL = 1 << 7;
        const DMA0 = 1 << 8;
        const DMA1 = 1 << 9;
        const DMA2 = 1 << 10;
        const DMA3 = 1 << 11;
        const KEYPAD = 1 << 12;
        const GAME_PAK = 1 << 13;
    }
}

impl IORegister for InterruptEnable {
    fn read(&self, byte: usize) -> u8 {
        match byte {
            0 => (self.bits >> 0) as u8,
            1 => (self.bits >> 8) as u8,
            2 => (self.bits >> 16) as u8,
            3 => (self.bits >> 24) as u8,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, _scheduler: &mut Scheduler, byte: usize, value: u8) {
        match byte {
            0 => self.bits = self.bits & !0x0000_00FF | (value as u32) << 0 & InterruptEnable::all().bits,
            1 => self.bits = self.bits & !0x0000_FF00 | (value as u32) << 8 & InterruptEnable::all().bits,
            2 => self.bits = self.bits & !0x00FF_0000 | (value as u32) << 16 & InterruptEnable::all().bits,
            3 => self.bits = self.bits & !0xFF00_0000 | (value as u32) << 24 & InterruptEnable::all().bits,
            _ => unreachable!(),
        }
    }
}

impl IORegister for InterruptMasterEnable {
    fn read(&self, byte: usize) -> u8 {
        match byte {
            0 => (self.bits >> 0) as u8,
            1 => (self.bits >> 8) as u8,
            2 => (self.bits >> 16) as u8,
            3 => (self.bits >> 24) as u8,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, _scheduler: &mut Scheduler, byte: usize, value: u8) {
        match byte {
            0 => self.bits = self.bits & !0x0000_00FF | (value as u32) << 0 & InterruptEnable::all().bits,
            1 => self.bits = self.bits & !0x0000_FF00 | (value as u32) << 8 & InterruptEnable::all().bits,
            2 => self.bits = self.bits & !0x00FF_0000 | (value as u32) << 16 & InterruptEnable::all().bits,
            3 => self.bits = self.bits & !0xFF00_0000 | (value as u32) << 24 & InterruptEnable::all().bits,
            _ => unreachable!(),
        }
    }
}

impl IORegister for InterruptRequest {
    fn read(&self, byte: usize) -> u8 {
        match byte {
            0 => (self.bits >> 0) as u8,
            1 => (self.bits >> 8) as u8,
            2 => (self.bits >> 16) as u8,
            3 => (self.bits >> 24) as u8,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, _scheduler: &mut Scheduler, byte: usize, value: u8) {
        match byte {
            0 => self.bits = self.bits & !0x0000_00FF | (value as u32) << 0 & InterruptEnable::all().bits,
            1 => self.bits = self.bits & !0x0000_FF00 | (value as u32) << 8 & InterruptEnable::all().bits,
            2 => self.bits = self.bits & !0x00FF_0000 | (value as u32) << 16 & InterruptEnable::all().bits,
            3 => self.bits = self.bits & !0xFF00_0000 | (value as u32) << 24 & InterruptEnable::all().bits,
            _ => unreachable!(),
        }
    }
}