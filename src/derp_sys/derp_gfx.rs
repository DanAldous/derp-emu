/*
 *
 */

pub struct GFX {
    address: u8,
}

impl GFX {
    fn get(&self, addr: u8) -> u8 {
        let index: usize = addr.into();
        //self.address[index]
        return self.address;
    }
    fn set(&mut self, addr: u8, data: u8) {
        self.address = data;
    }
    pub fn new() -> GFX {
        let addr: u8 = 0x0000;
        let gfx = GFX{address: addr};
        return gfx;
    }
}
