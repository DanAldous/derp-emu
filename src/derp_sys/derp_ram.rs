/*
 *
 */

pub struct RAM {
    address: [u8; 4096],
}

impl RAM {
    pub fn new() -> RAM {
        let mem: [u8; 4096] = [0; 4096]; 
        let ram = RAM{address:mem};
        return ram;
    }

    pub fn get(&self, addr: u16) -> u8 {
        let index: usize = addr.into();
        self.address[index]
    }

    fn set(&mut self, addr: u16, data: u8) {
        let index: usize = addr.into();
        self.address[index] = data;
    }
    fn load_font(&mut self) {
        let font:[u8;80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0,   // 0
            0x20, 0x60, 0x20, 0x20, 0x70,   // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0,   // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
            ];
        let mut i = 0;
        while i < 79 {
            let idx = i + 0x050;
            self.address[idx] = font[i];
            i+=1;
        }
    }
    pub fn load_cart(&mut self, size:usize , data: Vec<u8>) {
        let mut i = 0;
        while i < size {
            let idx = i + 0x200;
            self.address[idx] = data[i];
            i+=1;
        }
    }
}
