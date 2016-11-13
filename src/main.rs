extern crate sdl2;
extern crate congalife;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use congalife::{Game, State, advance};

struct TimerPulseEvent {}

pub fn main() {
    let screen_width = 1024;
    let screen_height = 1024;
    let game_size: usize = 512;
    let frame_time_in_ms = 1000 / 40 as u32;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let event_sys = sdl_context.event().unwrap();
    let timer_sys = sdl_context.timer().unwrap();

    event_sys.register_custom_event::<TimerPulseEvent>().unwrap();

    let window = video_subsystem.window("cromwell's gambol of fife", screen_width, screen_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, game_size as u32, game_size as u32).unwrap();

    let mut game: Game = Game::new(game_size);

    renderer.set_draw_color(Color::RGB(255, 0, 255));
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    let _timer;

    {
        let event_sys = event_sys.clone();
        let timer_callback = move || {
            event_sys.push_custom_event(TimerPulseEvent {}).unwrap();
            frame_time_in_ms
        };
        _timer = timer_sys.add_timer(frame_time_in_ms, Box::new(timer_callback));
    }


    'running: loop {
        for event in event_pump.poll_iter() {
            if event.is_user_event() {
                let _timer_pulse = event.as_user_event_type::<TimerPulseEvent>().unwrap();
                advance(&mut game);
            }

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let current = game.get_current();
            for y in 0..game_size {
                for x in 0..game_size {
                    let channel_value = match current[y * game.size + x] {
                        State::Dead => 0,
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