extern crate sdl2;
use crate::devices::Device;

use std::io::{Read, Result, Write, Error, ErrorKind};
use std::sync::mpsc;
use std::thread;

use sdl2::event::Event;

const WIDTH: u16 = 256;
const HEIGHT: u16 = 256;

#[derive(Debug)]
enum DisplayCommands {
    SetPixel(u16, u16, (u8,u8,u8)),
    Clear((u8,u8,u8)),
    Present,
    Quit
}

enum DisplayResponse {
    Ok,
    Error(String) // what the fuck why does this cause a warning
}

pub struct DisplayDevice {
    sender: mpsc::Sender<DisplayCommands>,
    receiver: mpsc::Receiver<DisplayResponse>,
    mem_buffer: [u8; 256],

    current_color: (u8,u8,u8),
    current_x: u16,
    current_y: u16
}

fn io_err<E: std::error::Error>(e: E) -> Error {
    Error::new(ErrorKind::Other, e.to_string())
}

impl DisplayDevice {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (resp_tx, resp_rx) = mpsc::channel();

        thread::spawn(move || {
            let context = sdl2::init().expect("failed to init sdl2");
            let video = context.video().unwrap();

            let window = video.window("putater", (WIDTH as u32) * 2, (HEIGHT as u32) * 2)
                .position_centered()
                .build()
                .unwrap();
                
            let mut canvas = window.into_canvas()
                .accelerated()
                .present_vsync() 
                .build()
                .unwrap();
            
            let texture_creator = canvas.texture_creator();
            let mut texture = texture_creator.create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32
            ).unwrap();
            
            let buffer_size = (WIDTH as usize) * (HEIGHT as usize);
            let mut pixel_buffer = vec![(0u8, 0u8, 0u8); buffer_size];

            let mut events = context.event_pump().unwrap();
            let mut running = true;
            let mut need_update = false;

            while running {
                for event in events.poll_iter() {
                    if let Event::Quit {..} = event {
                        running = false;
                        break;
                    }
                }
                
                while let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        DisplayCommands::SetPixel(x, y, (r,g,b)) => {
                            if x < WIDTH && y < HEIGHT {
                                pixel_buffer[(y * WIDTH + x) as usize] = (r, g, b);
                                need_update = true;
                            }
                        },
                        DisplayCommands::Clear((r,g,b)) => {
                            pixel_buffer.fill((r,g,b));
                            need_update = true;
                        }
                        DisplayCommands::Present => {
                            if need_update {
                                texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                                    for y in 0..HEIGHT as usize {
                                        for x in 0..WIDTH as usize {
                                            let offset = y * pitch + x * 3;
                                            let (r, g, b) = pixel_buffer[y * WIDTH as usize + x];

                                            buffer[offset] = r;       // red
                                            buffer[offset + 1] = g;   // green
                                            buffer[offset + 2] = b;   // blue
                                        }
                                    }
                                }).unwrap();

                                canvas.clear();
                                canvas.copy(&texture, None, None).expect("failed to render");
                                canvas.present();

                                need_update = false;
                            }
                        }
                        DisplayCommands::Quit => running = false,
                    }
                    let _ = resp_tx.send(DisplayResponse::Ok);
                }
                if !need_update {
                    thread::sleep(std::time::Duration::from_millis(5));
                }
            }
        });

        Self { 
            sender: cmd_tx,
            receiver: resp_rx,
            mem_buffer: [0; 256],

            current_color:(0,0,0),
            current_x:0,
            current_y:0
        }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, color: (u8,u8,u8)) -> Result<()> {
        self.sender.send(DisplayCommands::SetPixel(x, y, color)).map_err(io_err)?;
        self.receiver.recv().map_err(io_err).and_then(|res| {
            match res {
                DisplayResponse::Ok => Ok(()),
                DisplayResponse::Error(e) => Err(Error::new(ErrorKind::Other, e))
            }
        })
    }

    pub fn clear(&mut self, color: (u8,u8,u8)) -> Result<()> {
        self.sender.send(DisplayCommands::Clear(color)).map_err(io_err)?;
        self.receiver.recv().map_err(io_err).and_then(|res| {
            match res {
                DisplayResponse::Ok => Ok(()),
                DisplayResponse::Error(e) => Err(Error::new(ErrorKind::Other, e))
            }
        })
    }

    pub fn present(&mut self) -> Result<()> {
        self.sender.send(DisplayCommands::Present).map_err(io_err)?;
        self.receiver.recv().map_err(io_err).and_then(|res| {
            match res {
                DisplayResponse::Ok => Ok(()),
                DisplayResponse::Error(e) => Err(Error::new(ErrorKind::Other, e))
            }
        })
    }
    
    pub fn quit(&mut self) -> Result<()> {
        self.sender.send(DisplayCommands::Quit).map_err(io_err)?;
        self.receiver.recv().map_err(io_err).and_then(|res| {
            match res {
                DisplayResponse::Ok => Ok(()),
                DisplayResponse::Error(e) => Err(Error::new(ErrorKind::Other, e))
            }
        })
    }
}

impl Device for DisplayDevice {}

impl Read for DisplayDevice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = std::cmp::min(buf.len(), self.mem_buffer.len());
        buf[..len].copy_from_slice(&self.mem_buffer[..len]); 
        Ok(len)
    }
}

impl Write for DisplayDevice {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        for chunk in buf.chunks(2) {
            if let [value, instruction] = chunk {
                match instruction {
                    0 => {
                        self.current_color.0 = *value;
                    },
                    1 => {
                        self.current_color.1 = *value;
                    },
                    2 => {
                        self.current_color.2 = *value;
                    },
                    3 => {
                        self.current_x = *value as u16;
                    },
                    4 => {
                        self.current_y = *value as u16;
                    },
                    5 => {
                        self.set_pixel(self.current_x, self.current_y, self.current_color)?;
                    },
                    6 => {
                        self.set_pixel(self.current_x, self.current_y, (0,0,0))?;
                    },
                    7 => { 
                        self.present()?;
                    },
                    8 => {
                        self.clear((0,0,0))?;
                    },
                    _ => {}
                }
                println!("{} {} {:?}", self.current_x, self.current_y, self.current_color);
            }
        }
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
