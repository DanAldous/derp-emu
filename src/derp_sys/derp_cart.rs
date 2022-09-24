/*
 *
 */
use std::fs;


pub struct CART {
    size: usize,
    //data: [u8; 4096],
    data: Vec<u8>,
}

impl CART {
    pub fn new(rom: String) -> CART {
        //TODO: Try and open rom at string
        let bank = fs::read(rom).expect("Failed to load");
        let size = bank.len();
        let cart = CART{size: size,data: bank};
        println!("Hey, we loaded a rom! size is {}", size);
        return cart;
    }
}
