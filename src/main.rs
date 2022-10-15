/*
 *
 *
*/
mod derp_sys;
use derp_sys::*;
use ggez::*;
use std::time::Duration;


struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello ggez! dt = {}ns", self.dt.as_nanos());
        Ok(())
    }
}

fn main() {
    let state = State{
        dt: std::time::Duration::new(0,0),
    };


    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awsome_person")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);

    //pass card name to program here
    let rom_image = "assets/ibm.ch8".to_string();
    let mut system = Sys::new(rom_image);
    system.exec();
    println!("Hello, world!");
}