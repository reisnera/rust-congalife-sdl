extern crate sdl2;
extern crate congalife;

use std::thread;
use std::time;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use congalife::{Game, State};

pub fn main() {
    let screen_width = 512;
    let screen_height = 512;
    let game_size: usize = 128;
    let target_logic_frames_per_second = 40;
    let update_time_in_ms = 1000 / target_logic_frames_per_second as u64;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("cromwell's gambol of fife", screen_width, screen_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let game = Game::new(game_size, 0.06);

    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, game.size() as u32, game.size() as u32).unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 255));
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    let _state_update_thread = {
        let game = game.clone();
        let update_interval = time::Duration::from_millis(update_time_in_ms);
        
        thread::spawn(move || {
            loop {
                let before = time::Instant::now();
                game.advance_toroidally();
                let elapsed = time::Instant::now() - before;

                if elapsed < update_interval {
                    thread::sleep(update_interval - elapsed);
                }
            }
        })
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: Some(MouseButton::Left), x, y, .. } => {
                    let game_x = ((x as f32 / screen_width as f32) * game.size() as f32) as usize;
                    let game_y = ((y as f32 / screen_height as f32) * game.size() as f32) as usize;
                    game.set_cell(game_x, game_y, State::Alive);
                },
                _ => {}
            }
        }

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let current_guard = game.get_current_read_lock();
            for y in 0..game.size() {
                for x in 0..game.size() {
                    let channel_value = match current_guard[y * game.size() + x] {
                        State::Dead  => 0,
                        State::Alive => 255
                    };
                    let offset = y*pitch + x*3;
                    buffer[offset + 0] = channel_value;
                    buffer[offset + 1] = channel_value;
                    buffer[offset + 2] = channel_value;
                }
            }
        }).unwrap();
        
        renderer.clear();
        renderer.copy(&texture, None, Some(Rect::new(0, 0, screen_width, screen_height))).unwrap();
        renderer.present();
    }
}
