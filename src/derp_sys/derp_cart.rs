/*
 *
 */

pub struct CART {
    address: u8,
}

impl CART {
    fn get(&self, addr: u8) -> u8 {
        let index: usize = addr.into();
        //self.address[index]
        return self.address;
    }
    fn set(&mut self, addr: u8, data: u8) {
        //addr = data;
        self.address = data;
    }
    pub fn new() -> CART {
        let addr: u8 = 0x0000;
        let cart = CART{address: addr};
        return cart;
    }
}
