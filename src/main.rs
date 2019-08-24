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
mod level;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "high-scores" => {
                if args.len() == 3 {
                    high_scores(&args[2]);
                    return;
                }
            }
            "start" => {
                if args.len() == 4 {
                    start(&args[2], args[3].parse().unwrap());
                    return;
                }
            }
            _ => {}
        }
    }
    eprintln!("unknown usage");
    exit(1);
}


fn high_scores(episode: &str) {
    let path = Path::new(".").join("MINING").join(episode).join("high.dat");
    if !path.exists() {
        eprintln!("high scores file of {} does not exist", episode);
        exit(1);
    }
    let high_scores = high_score_table::load(&path).unwrap();
    for (i, high_score) in high_scores.iter().enumerate() {
        println!("{}. {:?}", i + 1, high_score);
    }
}


fn start(episode: &str, level_number: u8) {
    let palette_path = Path::new(".").join("MINING").join(episode).join(format!("{}.PAL", episode.to_uppercase()));
    if !palette_path.exists() {
        eprintln!("palette file of {} does not exist", episode);
        exit(1);
    }
    let tiles_path = Path::new(".").join("MINING").join(episode).join("TILE.DAT");
    if !tiles_path.exists() {
        eprintln!("tile file of {} does not exist", episode);
        exit(1);
    }
    let level_path = Path::new(".").join("MINING").join(episode).join(format!("LEVEL{:03}.BTN", level_number));
    if level_number != 0 && !level_path.exists() {
        eprintln!("level {:03} file of {} does not exist", level_number, episode);
        exit(1);
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem.window("", 1200, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(127, 127, 255));
    canvas.clear();
    canvas.present();

    let palette = palette::load_palette(&palette_path).unwrap();
    let tiles = tiles::load_tiles(&tiles_path).unwrap();
    let level = if level_number != 0 {
        level::load_level(&level_path).unwrap()
    } else {
        level::Level { width: 16, height: 16, map: (0..tiles.len() as u8).collect() }
    };

    let texture_creator = canvas.texture_creator();
    let mut tile_textures = vec![];
    for tile in tiles {
        let pixels: Vec<u8> = tile.data.iter().map(|&v| palette[v as usize].clone()).flatten().collect();
        let pixels = &pixels[..];
        let mut texture: Texture = texture_creator
            .create_texture_target(PixelFormatEnum::RGBA32, tile.width as u32, tile.height as u32)
            .unwrap();
        texture.set_blend_mode(BlendMode::Blend);
        texture.update(None, pixels, tile.width as usize * 4).unwrap();
        tile_textures.push((tile.width, tile.height, texture));
    }

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
        for y in 0..level.height {
            for x in 0..level.width {
                let pos = (y * level.width + x) as usize;
                if pos < level.map.len() {
                    let (width, height, tile_texture) = &tile_textures[level.map[pos] as usize];
                    canvas.copy(
                        tile_texture,
                        None,
                        Rect::new(x as i32 * *width as i32, y as i32 * *height as i32, *width as u32, *height as u32),
                    ).unwrap();
                }
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
