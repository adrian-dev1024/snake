extern crate sdl2;

use crate::game_context::GameContext;
use crate::game_context::GameState;
use crate::renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::DOT_SIZE_IN_PXS;
use crate::GRID_X_SIZE;
use crate::GRID_Y_SIZE;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Snake Game",
            (GRID_X_SIZE * DOT_SIZE_IN_PXS).try_into().unwrap(),
            (GRID_Y_SIZE * DOT_SIZE_IN_PXS).try_into().unwrap(),
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;
    let mut frame_counter = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W | Keycode::Up => context.move_up(),
                    Keycode::A | Keycode::Left => context.move_left(),
                    Keycode::S | Keycode::Down => context.move_down(),
                    Keycode::D | Keycode::Right => context.move_right(),
                    Keycode::Space => context.toggle_pause(),
                    Keycode::Return => context.select(),
                    _ => {}
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        if let GameState::Restart = context.state {
            context = GameContext::new();
        } else if let GameState::Quit = context.state {
            break 'running;
        }

        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&context)?;
    }

    Ok(())
}
