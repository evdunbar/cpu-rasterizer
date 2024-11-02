extern crate sdl2;
use sdl2::event::{Event, WindowEvent};

fn main() {
    // set up sdl
    let sdl_context = sdl2::init().unwrap();
    let video_subsytem = sdl_context.video().unwrap();

    let width = 800;
    let height = 600;

    let mut window = video_subsytem
        .window("Tiny rasterizer", width, height)
        .resizable()
        .allow_highdpi()
        .build()
        .unwrap();

    let mut mouse_x = 0;
    let mut mouse_y = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        window.set_size(width as u32, height as u32).unwrap()
                    }
                    _ => {}
                },
                Event::Quit { .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x;
                    mouse_y = y;
                }
                _ => {}
            }
        }
    }
}
