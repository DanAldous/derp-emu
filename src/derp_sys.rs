/*
 *
 */
mod derp_cpu;
mod derp_ram;
mod derp_gfx;
mod derp_keypad;
mod derp_audio;
mod derp_cart;

use derp_cpu::*;
use derp_ram::*;
use derp_gfx::*;
use derp_keypad::*;
use derp_audio::*;
use derp_cart::*;

pub struct Sys {
    mfg_label: String,
    cpu: CPU,
    ram: RAM,
    gfx: GFX,
    key: KEY,
    audio: AUDIO,
    cart: CART,
}

impl Sys {
    pub fn new(rom: String) -> Sys {
        let cpu = CPU::new();
        let ram = RAM::new();
        let gfx = GFX::new();
        let key = KEY::new();
        let audio = AUDIO::new();
        let cart = CART::new(rom);
        let system = Sys {mfg_label: "derp-CHIP8".to_string(), cpu: cpu, ram: ram, gfx: gfx, key: key, audio: audio, cart: cart};
        system
    }

    pub fn exec(&mut self) {
        let cart_size = self.cart.getSize();
        let cart_data = self.cart.getData();
        self.ram.load_cart(cart_size, cart_data);


        self.cpu.exec(&self.ram, &self.gfx);

    }

    pub fn reset(&self) {
    }
}
