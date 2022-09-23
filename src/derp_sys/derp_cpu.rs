/*
 *
 */

use super::derp_ram;

pub struct CPU {
    V: [u8;16],
    op: u16,
    idx: u16,
    pc: u16,
    delay_timer: u16,
    sound_timer: u16,
    stack: [u16;16],
    sp: u8,
    key: [u8;16],
}

impl CPU {
    pub fn new() -> CPU {
        let V:[u8; 16] = [0;16];
        let op:u16 = 0;
        let idx = 0;
        let pc:u16 = 0x200;//start address, 512 decimal
        let dt = 0;
        let st = 0;
        let stack:[u16; 16] = [0x0000;16];
        let sp = 0;
        let key:[u8;16] = [0x00;16];
        let cpu = CPU {V: V,
                        op: op,
                        idx: idx,
                        pc: pc,
                        delay_timer: dt,
                        sound_timer: st,
                        stack: stack,
                        sp: sp,
                        key: key
        };
        return cpu;
    }
    pub fn exec(&mut self, mut ram: &derp_ram::RAM) {
        self.op = self.next_op(ram);
    }

    fn next_op(&self, mut ram: &derp_ram::RAM) -> u16 {
        let op1 = ram.get(self.pc);
        let op2 = ram.get(self.pc+1);
        ((op1 as u16) << 8) | op2 as u16
    }
}

    
