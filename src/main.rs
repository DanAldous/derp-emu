/*
 *
 *
*/
mod derp_sys;
use derp_sys::*;

fn main() {
    //pass card name to program here
    let rom_image = "assets/ibm.ch8".to_string();
    let mut system = Sys::new(rom_image);
    system.exec();
    println!("Hello, world!");
}
