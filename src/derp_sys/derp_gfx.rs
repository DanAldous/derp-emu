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
    pub fn clear(&mut self) {
        //clear screen
        let mut i = 0;
        let max = 64 * 32;
        while i < max {
            self.address[i] = 0x00;
            i += 1;
        }
        self.dirty = true;
    }
    /*
        public void exec()
        {
            if (dirty)
            {
                this.dirty = false;
                for (int i = 0; i < 64; i++)
                    for (int j = 0; j < 32; j++)
                        xorPixel(i, j);

            }
            //TODO
        }
        public void stop()
        {
        }
        public void clear()
        {
            for (int i = 0; i < (64 * 32); i++)
                GFX[i] = 0x00;
            dirty = true;
        }
        public Boolean isDirty()
        {
            return dirty;
        }
        public Byte pixelAt(int x, int y)
        {
            return GFX[x + (64 * y)];//beware the stride
        }
        public void xorPixel(int x, int y)
        {
            if ((GFX[x + (64 * y)] ^= 1) != 0)//just xor the bit
                dirty = true;
        } 
     */
}
