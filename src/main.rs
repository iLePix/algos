use input::InputHandler;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::rect::{Point, Rect};
use vecm::vec::{Vec2i, Vec2u};
//use world::celo::Celo;
use std::time::Duration;

mod world;
mod input; 

use world::*;

use crate::input::Control;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Algos", 1600, 1000)
        .resizable()
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    
    let world_size = Vec2u::new(300, 300);//Vec2u::new(80, 50);
    let mut tile_size = 20; 
    let mut screen_size = Vec2u::new(1600, 1000);
    let mut camera = Camera::new();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut inputs = InputHandler::new();
    let mut world = World::new(world_size);
    world.p();

    //let celo = Celo::new(Vec2::fill(50.0), 1, 6);

    'running: loop {

        //INPUTS

        const OFFSET_INCREMENT: i32 =  5;
        
        //canvas.fill_rect(Rect::from_center(Point::new(400, 300), 80, 80))?;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //canvas.fill_rect(Rect::from_center(Point::new(400, 300), i as u32, i as u32))?;
        //canvas.clear(); 
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion {x, y, ..} => {
                    inputs.mouse_pos.x = x as u32;
                    inputs.mouse_pos.y = y as u32;
                },
                Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::SizeChanged(w, h) => {screen_size.x = w as u32; screen_size.y = h as u32},
                    sdl2::event::WindowEvent::Resized(w, h) => {screen_size.x = w as u32; screen_size.y = h as u32},
                    _ => (),
                },
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        inputs.set_key(key, true);
                    }
                },
                Event::KeyUp{ keycode, .. } => {
                    if let Some(key) = keycode {
                        inputs.set_key(key, false);
                    }
                },
                /*Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: true, .. } => {
                    camera.offset.x += offset_increment;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: true, .. } => {
                    camera.offset.x -= offset_increment;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: true, .. } => {
                    camera.offset.y += offset_increment;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: true,.. } => {
                    println!("go");
                    camera.offset.y -= offset_increment;
                },
                Event::KeyDown { keycode: Some(Keycode::Plus), repeat: true, .. } => {
                    camera.zoom_in();
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), repeat: false, .. } => {
                    camera.zoom_out();
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    println!("stop");
                },*/
                _ => {}
            }
        }






        if inputs.pressed(Control::ZoomOut) && tile_size >= 2 {
            tile_size -= 1;
        }
        if inputs.pressed(Control::ZoomIn) && tile_size < 20 {
            tile_size += 1;
        }
        if inputs.pressed(Control::Up) {
            camera.offset.y += OFFSET_INCREMENT;
        }
        if inputs.pressed(Control::Down) {
            camera.offset.y -= OFFSET_INCREMENT;
        }
        if inputs.pressed(Control::Left) {
            camera.offset.x += OFFSET_INCREMENT;
        }
        if inputs.pressed(Control::Right) {
            camera.offset.x -= OFFSET_INCREMENT;
        }





        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        const SELECTED_TICK: u8 = 32;

        //World render
        for x in 0..world.size.x {
            for y in 0..world.size.y {
                let mut tile = world.get_tile(Vec2u::new(x, y));
                //NEEED TO ADD OFFSET TO MOUSE COLLISION DETECTION
                if inputs.mouse_pos.x >= (x*tile_size) && inputs.mouse_pos.x < (x*tile_size + tile_size) && inputs.mouse_pos.y >= (y*tile_size) && inputs.mouse_pos.y < (y*tile_size + tile_size) { 
                 //NEEED TO ADD OFFSET TO MOUSE COLLISION DETECTION   
                    if tile.selected as i32 + SELECTED_TICK as i32 >= 255 {
                        tile.selected = 255;
                    } else {
                        tile.selected += SELECTED_TICK
                    }
                } else if tile.selected >= SELECTED_TICK {
                    tile.selected -= SELECTED_TICK;
                } else {
                    tile.selected = 0;
                }
                world.set_tile(Vec2u::new(x, y), tile);

                if tile.kind == TileType::Land {
                    canvas.set_draw_color(Color::RGB(0, 128, 0));
                } else if tile.kind == TileType::Water {
                    canvas.set_draw_color(Color::RGB(0, 0, 128));
                } else if tile.kind == TileType::Sand {
                    canvas.set_draw_color(Color::RGB(209, 188, 138));
                } else if tile.kind == TileType::Mountain {
                    canvas.set_draw_color(Color::RGB(32, 32, 32));
                } else if tile.kind == TileType::Snow {
                    canvas.set_draw_color(Color::RGB(250, 253, 254));
                }           
                canvas.fill_rect(Rect::new((x * tile_size) as i32 + camera.offset.x, (y * tile_size) as i32 + camera.offset.y, tile_size, tile_size))?;

                if tile.selected > 0 {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.fill_rect(Rect::from_center(Point::new((x * tile_size + tile_size / 2) as i32 + camera.offset.x, (y * tile_size + tile_size / 2) as i32 + camera.offset.y), (tile_size as f32 / 255 as f32 * tile.selected as f32) as u32, (tile_size as f32 / 255 as f32 * tile.selected as f32) as u32))?;
                }
            }
        }


        //CELO RENDER
        canvas.set_draw_color(Color::RGB(24, 220, 255));
        //canvas.(point)

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
