use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use vecm::vec::{Vec2i, Vec2u};
use std::time::Duration;

mod world; 

use world::*;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Algos", 1600, 1000)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    
    let world_size = Vec2u::new(80, 50);
    let tile_size = 20; 
    let mut mouse_pos = Vec2u::zero();
    let screen_size = Vec2u::new(1600, 1000);
    let mut camera = Camera::new();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    let mut world = World::new(world_size);
    world.p();

    'running: loop {
        //canvas.set_draw_color(Color::RGB(0, 0, 0));

        

        for x in 0..world.size.x {
            for y in 0..world.size.y {
                let mut tile = world.get_tile(Vec2u::new(x, y));
                if mouse_pos.x >= (x*tile_size) && mouse_pos.x < (x*tile_size + tile_size) && mouse_pos.y >= (y*tile_size) && mouse_pos.y < (y*tile_size + tile_size){ 
                    
                    if tile.selected != 255 {
                        tile.selected += 17;
                    }
                } else if tile.selected > 0 {
                    tile.selected -= 17;
                }
                world.set_tile(Vec2u::new(x, y), tile);

                if tile.kind == TileType::Land {
                    canvas.set_draw_color(Color::RGB(0, 128, 0));
                } else if tile.kind == TileType::Water {
                    canvas.set_draw_color(Color::RGB(0, 0, 128));
                }  else if tile.kind == TileType::Mountain {
                    canvas.set_draw_color(Color::RGB(32, 32, 32));
                }             
                canvas.fill_rect(Rect::new((x * tile_size) as i32, (y * tile_size) as i32, tile_size, tile_size))?;


                if tile.selected > 0 {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.fill_rect(Rect::from_center(Point::new((x * tile_size + tile_size / 2) as i32, (y * tile_size + tile_size / 2) as i32), (tile_size as f32 / 255 as f32 * tile.selected as f32) as u32, (tile_size as f32 / 255 as f32 * tile.selected as f32) as u32))?;
                }
            }
        }


        //canvas.fill_rect(Rect::from_center(Point::new(400, 300), 80, 80))?;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //canvas.fill_rect(Rect::from_center(Point::new(400, 300), i as u32, i as u32))?;
        //canvas.clear(); 
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion {x, y, ..} => {
                    mouse_pos.x = x as u32;
                    mouse_pos.y = y as u32;
                },
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    camera.offset.y -= 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    camera.offset.x += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    camera.offset.y -= 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    camera.offset.y += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Plus), repeat: false, .. } => {
                    camera.zoom_in();
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), repeat: false, .. } => {
                    camera.zoom_out();
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

pub struct Camera {
    pub offset: Vec2i,
    pub zoom: i8,    
}

impl Camera{
    pub fn new() -> Self{
        Self {offset: Vec2i::zero(), zoom: 1}
    }

    pub fn zoom_in(&mut self) {
        if self.zoom < 4 {
            self.zoom *= 2;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom > 1 {
            self.zoom /= 2;
        }
    }
}