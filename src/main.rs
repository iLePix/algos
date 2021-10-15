use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use vecm::vec::Vec2u;
use std::time::Duration;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Algos", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    
    let world_size = Vec2u::new(16, 12);
    let mut mouse_pos = Vec2u::zero();
    let screen_size = Vec2u::new(800, 600);    

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 80;
        //canvas.set_draw_color(Color::RGB(0, 0, 0));

    

        for x in 0..world_size.x {
            for y in 0..world_size.y {
                if mouse_pos.x >= (x*50) && mouse_pos.x < (x*50 + 50) && mouse_pos.y >= (y*50) && mouse_pos.y < (y*50 + 50){ 
                    canvas.set_draw_color(Color::RGB((x*8) as u8, (y*8) as u8, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                
                canvas.fill_rect(Rect::new((x * 50) as i32, (y * 50) as i32, 50, 50))?;
            }
        }


        //canvas.fill_rect(Rect::from_center(Point::new(400, 300), 80, 80))?;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //canvas.fill_rect(Rect::from_center(Point::new(400, 300), i as u32, i as u32))?;
        //canvas.clear(); ?????????????????????????????
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion {x, y, ..} => {
                    mouse_pos.x = x as u32;
                    mouse_pos.y = y as u32;
                    println!("x: {}, y: {}", x, y);
                },
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
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