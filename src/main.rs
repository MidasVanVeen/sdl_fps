extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

mod utils;
mod vectors;
use utils::*;
pub(crate) use vectors::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const MAP_SIZE: i32 = 8;

const MAPDATA: [u8; (MAP_SIZE * MAP_SIZE) as usize] = [
    1, 2, 1, 1, 1, 1, 1, 1, 2, 0, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 3, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 2, 0, 4, 4, 0, 1, 1, 0, 0, 0, 4, 0, 0, 1, 1, 0, 3, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

struct State {
    canvas: WindowCanvas,
    quit: bool,
    pos: V2<f32>,
    dir: V2<f32>,
    plane: V2<f32>,
}

#[derive(Debug)]
struct DDAHit {
    val: u8,
    side: i32,
    pos: V2<f32>,
}

fn u32_to_u8_array(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;

    [b1, b2, b3, b4]
}

impl State {
    fn verline(&mut self, x: i32, y0: i32, y1: i32, color: u32) {
        let b = u32_to_u8_array(color);
        self.canvas
            .set_draw_color(Color::RGBA(b[0], b[1], b[2], b[3]));
        self.canvas
            .draw_line(Point::new(x, y0), Point::new(x, y1))
            .unwrap();
    }

    fn render(&mut self) {
        for x in 0..SCREEN_WIDTH {
            let xcam: f32 = (2.0 * (x as f32 / SCREEN_WIDTH as f32)) - 1.0;
            let dir: V2<f32> = V2::new(
                self.dir.x + self.plane.x * xcam,
                self.dir.y + self.plane.y * xcam,
            );
            let pos: V2<f32> = self.pos;
            let mut ipos: V2<i32> = V2::new(pos.x.floor() as i32, pos.y.floor() as i32);

            let deltadist: V2<f32> = V2::new(
                if dir.x.abs() < 10_f32.powf(-20.0) {
                    10_i32.pow(30) as f32
                } else {
                    (1.0 / dir.x).abs()
                },
                if dir.y.abs() < 10_f32.powf(-20.0) {
                    10_i32.pow(30) as f32
                } else {
                    (1.0 / dir.y).abs()
                },
            );

            let mut sidedist: V2<f32> = V2::new(
                deltadist.x
                    * (if dir.x < 0.0 {
                        pos.x - ipos.x as f32
                    } else {
                        ipos.x as f32 + 1.0 - pos.x
                    }),
                deltadist.y
                    * (if dir.y < 0.0 {
                        pos.y - ipos.y as f32
                    } else {
                        ipos.y as f32 + 1.0 - pos.y
                    }),
            );

            let step: V2<i32> = V2::new(utils::sign::<i32>(dir.x), utils::sign::<i32>(dir.y));

            let mut hit = DDAHit {
                val: 0,
                side: 0,
                pos: V2::new(0.0, 0.0),
            };

            while hit.val == 0 {
                if sidedist.x < sidedist.y {
                    sidedist.x += deltadist.x;
                    ipos.x += step.x;
                    hit.side = 0;
                } else {
                    sidedist.y += deltadist.y;
                    ipos.y += step.y;
                    hit.side = 1;
                }

                assert!(ipos.x >= 0 && ipos.x < MAP_SIZE && ipos.y >= 0 && ipos.y < MAP_SIZE);

                hit.val = MAPDATA[(ipos.y * MAP_SIZE as i32 + ipos.x) as usize];
            }

            let mut color: u32 = match hit.val {
                1 => 0xFF0000FF,
                2 => 0xFF00FF00,
                3 => 0xFFFF0000,
                4 => 0xFFFF00FF,
                _ => 0x0,
            };

            if hit.side == 1 {
                let br: u32 = ((color & 0xFF00FF) * 0xC0) >> 8;
                let g: u32 = ((color & 0x00FF00) * 0xC0) >> 8;
                color = 0xFF000000 | (br & 0xFF00FF) | (g & 0x00FF00);
            }

            hit.pos = V2::new(pos.x + sidedist.x, pos.y + sidedist.y);

            let dperp: f32 = if hit.side == 0 {
                sidedist.x - deltadist.x
            } else {
                sidedist.y - deltadist.y
            };

            let h: i32 = (SCREEN_HEIGHT as f32 / dperp).round() as i32;
            let y0: i32 = max(SCREEN_HEIGHT / 2 - h / 2, 0);
            let y1: i32 = min(SCREEN_HEIGHT / 2 + h / 2, SCREEN_HEIGHT - 1);

            self.verline(x, 0, y0, 0x0);
            self.verline(x, y0, y1, color);
            self.verline(x, y1, SCREEN_HEIGHT - 1, 0x0);
        }
    }

    #[allow(dead_code)]
    fn rotate(&mut self, rot: f32) {
        let d: V2<f32> = self.dir;
        let p: V2<f32> = self.plane;
        self.dir.x = d.x * rot.cos() - d.y * rot.sin();
        self.dir.y = d.x * rot.sin() + d.y * rot.cos();
        self.plane.x = p.x * rot.cos() - p.y * rot.sin();
        self.plane.y = p.x * rot.sin() + p.y * rot.cos();
    }

    fn move_forward(&mut self, movespeed: f32) {
        // if MAPDATA[((self.pos.y+movespeed) as i32 * MAP_SIZE + ((self.pos.x+movespeed) as i32)) as usize] == 0 {
        self.pos.x += self.dir.x * movespeed;
        self.pos.y += self.dir.y * movespeed;
        // }
    }

    fn move_sideways(&mut self, movespeed: f32) {
        self.pos.x += self.dir.y * movespeed;
        self.pos.y += -self.dir.x * movespeed;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let window = video_subsystem
        .window("DEMO", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let mut state: State = State {
        canvas,
        pos: V2::new(2.0, 2.0),
        dir: V2::new(-1.0, 0.1).normalize(),
        plane: V2::new(0.0, 0.66),
        quit: false,
    };

    while !state.quit {
        state.canvas.set_draw_color(Color::BLACK);
        state.canvas.clear();

        let _rotspeed: f32 = 0.00128;
        let _movespeed: f32 = 0.0024;

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(k), ..
                } => match k {
                    Keycode::Escape => {
                        state.quit = true;
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        state.render();

        let mouse_state = event_pump.relative_mouse_state();
        state.rotate(_rotspeed * mouse_state.x() as f32);
        sdl_context
            .mouse()
            .warp_mouse_in_window(state.canvas.window(), 1280 / 2, 720 / 2);

        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            state.move_sideways(-_movespeed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            state.move_sideways(_movespeed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            state.move_forward(_movespeed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            state.move_forward(-_movespeed);
        }

        state.canvas.present();
    }
}
