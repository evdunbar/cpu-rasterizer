use cpu_rasterizer::rasterizer::*;
extern crate sdl2;
use sdl2::{
    event::{Event, WindowEvent},
    pixels::PixelFormatEnum,
    render::BlendMode,
    surface::Surface,
};
use std::time::Instant;

fn main() {
    // set up sdl
    let sdl_context = sdl2::init().unwrap();
    let video_subsytem = sdl_context.video().unwrap();

    let mut width = 800;
    let mut height = 600;

    let window = video_subsytem
        .window("Tiny rasterizer", width, height)
        .resizable()
        .allow_highdpi()
        .build()
        .unwrap();
    let mut maybe_draw_surface: Option<Surface> = None;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut mouse_x = 0;
    let mut mouse_y = 0;

    let mut last_frame_start = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(new_width, new_height) => {
                        maybe_draw_surface = None;
                        width = new_width as u32;
                        height = new_height as u32;
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

        if maybe_draw_surface.is_none() {
            let mut draw_surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
            draw_surface.set_blend_mode(BlendMode::None).unwrap();
            maybe_draw_surface = Some(draw_surface)
        }

        let dt = last_frame_start.elapsed();
        println!("{}", dt.as_secs_f64());
        last_frame_start = Instant::now();

        if let Some(ref mut draw_surface) = maybe_draw_surface {
            let mut color_buffer = ImageView::from_pixel_buffer(
                draw_surface.without_lock_mut().unwrap(),
                width,
                height,
            );
            color_buffer.clear(Vec4::new(0.8, 0.9, 1.0, 1.0));

            let rect = draw_surface.rect();
            let window_surface = &mut window.surface(&event_pump).unwrap();
            let _ = draw_surface.blit(rect, window_surface, rect);
            window_surface.update_window().unwrap();
        }
    }
}
