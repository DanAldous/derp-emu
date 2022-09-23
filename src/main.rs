/*
 *
 *
*/
mod derp_sys;
use derp_sys::*;

fn main() {
    let mut system = Sys::new();
    system.exec();
    println!("Hello, world!");
}
