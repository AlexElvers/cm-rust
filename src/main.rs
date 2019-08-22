use std::env;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Texture;

mod high_score_table;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match args[1].as_ref() {
            "high-scores" => {
                if args.len() == 3 {
                    high_scores(args[2].as_ref());
                    return;
                }
            }
            "start" => {
                if args.len() == 3 {
                    start(args[2].as_ref());
                    return;
                }
            }
            _ => {}
        }
    }
    eprintln!("unknown usage");
    exit(1);
}


fn high_scores(level: &str) {
    let path = Path::new(".").join("MINING").join(level).join("high.dat");
    if !path.exists() {
        eprintln!("high scores file of {} does not exist", level);
        exit(1);
    }
    let high_scores = high_score_table::load(&path).unwrap();
    for (i, high_score) in high_scores.iter().enumerate() {
        println!("{}. {:?}", i + 1, high_score);
    }
}


fn start(level: &str) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem.window("", 600, 400)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(127, 127, 255));
    canvas.clear();
    canvas.present();

    let pixels = &mut [0; 32 * 32 * 4];
    for (i, v) in pixels.chunks_exact_mut(3).enumerate() {
        let x = i % 32;
        let y = i / 32;
        if x == y || x + y == 31 {
            v[0] = 255;
        } else if x == 0 || y == 0 || x == 31 || y == 31 {
            v[1] = 255;
        }
    }

    let texture_creator = canvas.texture_creator();
    let mut texture: Texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();
    texture.update(Rect::new(0, 0, 32, 32), pixels, 32 * 3).unwrap();

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
                _ => {}
            }
        }

        for y in (0..32 * 8).step_by(32) {
            for x in (0..32 * 16).step_by(32) {
                canvas.copy(&texture, None, Rect::new(x, y, 32, 32)).unwrap();
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
