/*
 *
 *
*/
mod derp_sys;
extern crate sdl2;
use derp_sys::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("DerpEMU Test", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0,255,255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i+1)%255;
        canvas.set_draw_color(Color::RGB(i,64,255-i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0,1_000_000_000u32 / 60));
    }
    //pass card name to program here
    let rom_image = "assets/ibm.ch8".to_string();
    let mut system = Sys::new(rom_image);
    system.exec();
    println!("Hello, world!");
}
