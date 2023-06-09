mod chip8;
mod timers;
mod operations_set;
mod config;
extern crate sdl2;
extern crate rand;

use ::chip8::config::Args;
use config::parse_display;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;

use crate::chip8::{Chip8, ProgramType};
use clap::Parser;



fn main() -> Result<(), String> {
    let mut chip = Chip8::new();
    let file = Args::parse().file;
    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;
    let display_config = parse_display();

    let window = video_subsystem
        .window("rust-sdl2 demo", display_config.window_width, display_config.window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, display_config.window_width, display_config.window_height)
        .map_err(|e| e.to_string())?;
    

    chip.load_program(ProgramType::Main(file.as_str()), None, None).expect("Error loading program: ");
    loop {
        if let Err(eop) = chip.execute_cycle() {
            println!("Program terminated with status: {:?}", eop);
            break;
        }
        
        let vf = chip.get_register_value(15);
        if vf & 0x80 == 0x80 {
            // update screen
            'mainloop: loop {
                for event in sdl2_context.event_pump()?.poll_iter() {
                    match event {
                        Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        }
                        | Event::Quit { .. } => break 'mainloop,
                        _ => {}
                    }
                }
                let mut rects = Vec::new();
                for (no, row) in chip.get_gfx().iter().enumerate() {
                    for (px_no, px8) in row.iter().enumerate() {
                        
                        for ind_px in 0..8 {
                            
                            
                            if (*px8 >> ind_px) & 0x01 == 0x01 {
                                rects.push(Rect::new((((px_no*8 + ind_px) as u32)*display_config.window_width / 64) as i32, 
                                ((no as u32)*display_config.window_height / 32) as i32,
                                display_config.window_width / 64,
                                display_config.window_height / 32));
                            }
                        }
                    }
                }
                canvas
                    .with_texture_canvas(&mut texture, |texture_canvas| {
                        //texture_canvas.clear();
                        texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                        texture_canvas
                            .fill_rects(&rects)
                            .expect("could not fill rect");
                    })
                    .map_err(|e| e.to_string())?;
                canvas.set_draw_color(Color::RGBA(0, 255, 0, 0));
                canvas.clear();
                canvas.copy_ex(
                    &texture,
                    None,
                    None,
                    0.0,
                    None,
                    false,
                    false,
                )?;
                canvas.present();
            }


            // put the draw flag down
            chip.set_register_value(15, vf & 0x7F);
        }
    }

    Ok(())
}