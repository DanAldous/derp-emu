/*
 *
 */
mod derp_cpu;
mod derp_ram;
mod derp_gfx;
mod derp_keypad;
mod derp_cart;

use derp_cpu::*;
use derp_ram::*;
use derp_gfx::*;
use derp_keypad::*;
use derp_cart::*;

pub struct Sys {
    mfg_label: String,
    cpu: CPU,
    ram: RAM,
    gfx: GFX,
    key: KEY,
    cart: CART,
}

impl Sys {
    pub fn new() -> Sys {
        let cpu = CPU::new();
        let ram = RAM::new();
        let gfx = GFX::new();
        let key = KEY::new();
        let cart = CART::new();
        let system = Sys {mfg_label: "dCHIP8".to_string(), cpu: cpu, ram: ram, gfx: gfx, key: key, cart: cart};
        system
    }

    pub fn exec(&self) {
    }

    pub fn reset(&self) {
    }
}
