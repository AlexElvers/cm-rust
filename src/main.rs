use std::env;
use std::path::Path;
use std::process::exit;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, WindowCanvas};
use crate::objects::Object;
use crate::level::{Cell, Direction, Level};

mod high_score_table;
mod tiles;
mod palette;
mod level;
mod objects;


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
            "tiles" => {
                if args.len() == 3 {
                    tiles(&args[2]);
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


fn tiles(episode: &str) {
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

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem.window("", 660, 500)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(127, 127, 255));
    canvas.clear();
    canvas.present();

    let palette = palette::load_palette(&palette_path).unwrap();
    let tiles = tiles::load_tiles(&tiles_path).unwrap();

    let texture_creator = canvas.texture_creator();
    let mut tile_textures = vec![];
    for tile in &tiles {
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
        for y in 0..tiles.len() / 16 + 1 {
            for x in 0..16 {
                let pos = (y * 16 + x) as usize;
                if pos < tiles.len() {
                    let (width, height, tile_texture) = &tile_textures[pos as usize];
                    canvas.copy(
                        tile_texture,
                        None,
                        Rect::new(x as i32 * *width as i32, y as i32 * *height as i32, *width as u32, *height as u32),
                    ).unwrap();
                }
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 15));
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
    if !level_path.exists() {
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
    let mut level = level::load_level(&level_path).unwrap();

    for pos in 0..level.map.len() {
        let mut cell = level.map[pos];
        if let Cell { x, y, object: Some(ref mut object), .. } = cell {
            match object {
                Object::Dirt {
                    ref mut north,
                    ref mut east,
                    ref mut south,
                    ref mut west,
                } => {
                    if x >= 1 {
                        if let Some(neighbor) = level.cell(x - 1, y) {
                            *west = neighbor.is_transparent();
                        }
                    }
                    if x < level.width - 1 {
                        if let Some(neighbor) = level.cell(x + 1, y) {
                            *east = neighbor.is_transparent();
                        }
                    }
                    if y >= 1 {
                        if let Some(neighbor) = level.cell(x, y - 1) {
                            *north = neighbor.is_transparent();
                        }
                    }
                    if y < level.height - 1 {
                        if let Some(neighbor) = level.cell(x, y + 1) {
                            *south = neighbor.is_transparent();
                        }
                    }
                    level.map[pos] = cell;
                }
                Object::Wall {
                    ref mut north,
                    ref mut east,
                    ref mut south,
                    ref mut west,
                } => {
                    if x >= 1 {
                        if let Some(neighbor) = level.cell(x - 1, y) {
                            *west = match neighbor.object {
                                Some(Object::Wall { .. }) => false,
                                _ => true
                            };
                        }
                    }
                    if x < level.width - 1 {
                        if let Some(neighbor) = level.cell(x + 1, y) {
                            *east = match neighbor.object {
                                Some(Object::Wall { .. }) => false,
                                _ => true
                            };
                        }
                    }
                    if y >= 1 {
                        if let Some(neighbor) = level.cell(x, y - 1) {
                            *north = match neighbor.object {
                                Some(Object::Wall { .. }) => false,
                                _ => true
                            };
                        }
                    }
                    if y < level.height - 1 {
                        if let Some(neighbor) = level.cell(x, y + 1) {
                            *south = match neighbor.object {
                                Some(Object::Wall { .. }) => false,
                                _ => true
                            };
                        }
                    }
                    level.map[pos] = cell;
                }
                _ => (),
            }
        }
    }

    // let iter_mut = level.map.iter_mut();
    // for cell in iter_mut {
    //     // cell.update(&level);
    //     let a = &level.map[0];
    // }

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
    let mut tick_number = 0u8;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
                _ => {}
            }
        }

        let mut move_left = false;
        let mut move_right = false;
        let mut move_up = false;
        let mut move_down = false;
        for keycode in event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode) {
            match keycode {
                Keycode::Left => move_left = true,
                Keycode::Right => move_right = true,
                Keycode::Up => move_up = true,
                Keycode::Down => move_down = true,
                _ => {}
            }
        }

        for y in 0..level.height {
            for x in 0..level.width {
                if let Some(&cell) = level.cell(x, y) {
                    if let Cell { object: Some(Object::Player), moving_in_from: Some(_), .. } = cell {
                        level.set_cell(x, y, Cell {
                            moving_in_from: None,
                            changed_in_current_tick: true,
                            ..cell
                        })
                    }
                }
            }
        }

        for y in 0..level.height {
            for x in 0..level.width {
                if let Some(&cell) = level.cell(x, y) {
                    if let Cell { object: Some(Object::Player), changed_in_current_tick: false, .. } = cell {
                        if move_left && x >= 1 {
                            move_player_if_possible(&mut level, x, y, Direction::East);
                        } else if move_right && x < level.width - 1 {
                            move_player_if_possible(&mut level, x, y, Direction::West);
                        } else if move_up && y >= 1 {
                            move_player_if_possible(&mut level, x, y, Direction::South);
                        } else if move_down && y < level.height - 1 {
                            move_player_if_possible(&mut level, x, y, Direction::North);
                        }
                    }
                }
            }
        }

        for y in 0..level.height {
            for x in 0..level.width {
                if let Some(&cell) = level.cell(x, y) {
                    level.set_cell(x, y, Cell { changed_in_current_tick: false, ..cell })
                }
            }
        }

        canvas.clear();
        // Draw background.
        for y in 0..level.height {
            for x in 0..level.width {
                if let Some(cell) = level.cell(x, y) {
                    if let Some(object) = cell.object {
                        if object.is_transparent() {
                            draw_object(&mut canvas, &tile_textures, x, y, None, Object::Empty, tick_number)
                        }
                    } else {
                        draw_object(&mut canvas, &tile_textures, x, y, None, Object::Empty, tick_number);
                    }
                }
            }
        }
        // Draw (possibly transparent) foreground.
        for y in 0..level.height {
            for x in 0..level.width {
                if let Some(cell) = level.cell(x, y) {
                    if let Some(object) = cell.object {
                        draw_object(&mut canvas, &tile_textures, x, y, cell.moving_in_from, object, tick_number);
                    }
                }
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 15));
        tick_number = (tick_number + 1) % 12;
    }
}

fn move_player_if_possible(level: &mut Level, x: u16, y: u16, moving_in_from: Direction) {
    if let Some(&cell) = level.cell(x, y) {
        let x_new = (x as i16 + match moving_in_from {
            Direction::West => 1,
            Direction::East => -1,
            _ => 0
        }) as u16;
        let y_new = (y as i16 + match moving_in_from {
            Direction::North => 1,
            Direction::South => -1,
            _ => 0
        }) as u16;
        if let Some(&neighbor) = level.cell(x_new, y_new) {
            if neighbor.can_be_entered() {
                level.set_cell(x, y, Cell {
                    object: None,
                    post_occupied: true,
                    changed_in_current_tick: true,
                    ..cell
                });
                level.set_cell(x_new, y_new, Cell {
                    object: Some(Object::Player),
                    pre_occupied: true,
                    changed_in_current_tick: true,
                    moving_in_from: Some(moving_in_from),
                    ..neighbor
                });
            }
        }
    }
}

fn draw_object(mut canvas: &mut WindowCanvas, tile_textures: &Vec<(u16, u16, Texture)>, x: u16, y: u16, offset: Option<Direction>, object: Object, tick_number: u8) {
    draw_tile(&mut canvas, &tile_textures, x, y, offset, object.tile_number(tick_number) as usize);
}

fn draw_tile(canvas: &mut WindowCanvas, tile_textures: &Vec<(u16, u16, Texture)>, x: u16, y: u16, offset: Option<Direction>, tile_number: usize) {
    let (width, height, tile_texture) = &tile_textures[tile_number];
    canvas.copy(
        tile_texture,
        None,
        Rect::new(
            x as i32 * *width as i32 + match offset {
                Some(Direction::West) => -(*width as i32) / 2,
                Some(Direction::East) => (*width as i32) / 2,
                _ => 0,
            },
            y as i32 * *height as i32 + match offset {
                Some(Direction::North) => -(*height as i32) / 2,
                Some(Direction::South) => (*height as i32) / 2,
                _ => 0,
            },
            *width as u32,
            *height as u32,
        ),
    ).unwrap();
}

