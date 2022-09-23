/*
 *
 */


pub struct CPU {
    r1: u8,
    r2: u8,
    r3: u8,
    r4: u8,
}

impl CPU {
    fn op(&self, opcode: u8) {
        if opcode == 0x0000 {
            return;
        } else if opcode == 0x0001 {
            return;
        } else if opcode == 0x0002 {
            return;
        }
    }
    pub fn new() -> CPU {
        let cpu = CPU {r1:0x0000,r2:0x0000,r3:0x0000,r4:0x0000};
        return cpu;
    }
}

    
