/*
 *
 */

pub struct KEY {
    address: u8,
}

impl KEY {
    fn get(&self, addr: u8) -> u8 {
        let index: usize = addr.into();
        //self.address[index]
        return self.address;
    }
    fn set(&mut self, addr: u8, data: u8) {
        self.address = data;
    }
    pub fn new() -> KEY {
        let addr: u8 = 0x0000;
        let key = KEY{address: addr};
        return key;
    }
}
