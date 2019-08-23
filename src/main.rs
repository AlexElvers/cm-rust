use std::env;
use std::path::Path;
use std::process::exit;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture};

mod high_score_table;
mod tiles;
mod palette;


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
    let palette_path = Path::new(".").join("MINING").join(level).join(format!("{}.PAL", level.to_uppercase()));
    if !palette_path.exists() {
        eprintln!("palette file of {} does not exist", level);
        exit(1);
    }
    let tiles_path = Path::new(".").join("MINING").join(level).join("TILE.DAT");
    if !tiles_path.exists() {
        eprintln!("tile file of {} does not exist", level);
        exit(1);
    }

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

    let palette = palette::load_palette(&palette_path);

    let tiles = tiles::load_tiles(&tiles_path);
    let pixels: Vec<u8> = tiles[180].iter().map(|&v| palette[v as usize].clone()).flatten().collect();
    let pixels = &pixels[..];

    let width = 40;
    let height = 40;
    let texture_creator = canvas.texture_creator();
    let mut texture: Texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGBA32, width, height)
        .unwrap();
    texture.set_blend_mode(BlendMode::Blend);
    texture.update(Rect::new(0, 0, width, height), pixels, width as usize * 4).unwrap();

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
                _ => {}
            }
        }

        canvas.clear();
        for y in (0..height * 8).step_by(height as usize) {
            for x in (0..width * 16).step_by(width as usize) {
                canvas.copy(&texture, None, Rect::new(x as i32, y as i32, width, height)).unwrap();
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
