use sdl2::{video::Window, EventPump};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::{keyboard::Keycode, mouse::MouseButton};

const WINDOW_SIZE: u32 = 1000;

fn init_window() -> (Window, EventPump) {
    let context = sdl2::init().expect("Failed to initailize SDL context");
    let video_subsystem = context.video().expect("Failed to initialize video subsystem");
    let window = video_subsystem
        .window("Rust Chess", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .expect("Failed to create window");
    let mut event_pump = context.event_pump().expect("Failed to initialize event pump");
    return (window, event_pump);
}

fn main() {
    let (window, mut event_pump) = init_window();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    break 'running
                }
                _ => {}
            }
        }
    }
}
