/*
 *
 */


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
        let op = 0;
        let idx = 0;
        let pc = 0x200;//start address, 512 decimal
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
    
    fn op(&self, opcode: u8) {
        if opcode == 0x0000 {
            return;
        } else if opcode == 0x0001 {
            return;
        } else if opcode == 0x0002 {
            return;
        }
    }
}

    
