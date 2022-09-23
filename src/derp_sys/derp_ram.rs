/*
 *
 */

pub struct RAM {
    address: [u8; 80],
}

impl RAM {
    fn get(&self, addr: u8) -> u8 {
        let index: usize = addr.into();
        self.address[index]
    }
    fn set(&mut self, addr: u8, data: u8) {
        let index: usize = addr.into();
        self.address[index] = data;
    }
    pub fn new() -> RAM {
        let mem: [u8; 80] = [0; 80]; 
        let ram = RAM{address:mem};
        return ram;
    }
}
