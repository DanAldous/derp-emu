/*
 *
 */

pub struct GFX {
    address: [u8;64*32],
    dirty: bool,
}

impl GFX {
    pub fn new() -> GFX {
        let addr: [u8;64*32] = [0x0000;64*32];
        let dirty = true;
        let gfx = GFX{address: addr, dirty: dirty};
        return gfx;
    }
    fn get(&self, addr: u8) -> u8 {
        let index: usize = addr.into();
        self.address[index]
        //return self.address;
    }
    fn set(&mut self, addr: u8, data: u8) {
        let index: usize = addr.into();
        self.address[index] = data;
    }
}
