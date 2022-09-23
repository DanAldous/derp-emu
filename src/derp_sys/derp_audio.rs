/*
 *
 */

pub struct IO {
    address: u8,
}

impl IO {
    fn get(&self, addr: u8) -> u8 {
        let index: usize = addr.into();
        self.address[index]
    }
    fn set(&self, addr: u8, data: u8) {
        addr = data;
    }
    pub fn new() -> IO {
        let addr: u8 = 0x0000;
        let io = IO{address: addr};
        return io;
    }
}
