extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::event::{QuitEvent, poll_event, KeyDownEvent};
use sdl2::rect::{Rect};
use sdl2::timer::{delay, get_ticks};
use std::cmp::{max};

struct Position {
    x: f32,
    y: f32,
}

struct Movement {
    dx: f32,
    dy: f32
}

struct Player {
    pos: Position,
    mov: Movement
}

struct Ball {
    pos: Position,
    mov: Movement
}

struct Background;

trait Renderable {
    fn draw(&self, renderer:  &sdl2::render::Renderer<Window>);
}

// TODO: Player-Ball Collision detection
// TODO: Score when ball crosses line
// TODO: vsync? Setting frame rate to something good?

impl Renderable for Player {
    fn draw(&self, renderer:  &sdl2::render::Renderer<Window>) {
        // Set the drawing color to a darker blue.
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 153, 204));
        // Create a smaller centered Rect, filling it in the same dark blue.
        let l_bat = Rect::new(self.pos.x as i32, self.pos.y as i32, 30, 120);
        let _ = match renderer.fill_rect(&l_bat) {
            Ok(_)    => {},
            Err(err) => fail!("failed to draw rect: {}", err) 
        };
    }
}

impl Player {
    fn update(& mut self, timestep: uint) {
        if (self.pos.y <= 0.0 && self.mov.dy < 0.0) ||
           (self.pos.y + 120.0 > 480.0 && self.mov.dy > 0.0) {
            self.mov.dy = 0.0;
        }
        self.pos.x += self.mov.dx;
        self.pos.y += self.mov.dy;
    }
}

impl Renderable for Ball {
    fn draw(&self, renderer: &sdl2::render::Renderer<Window>) {
        // Set the drawing color to a darker blue.
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 153, 204));
        let ball = Rect::new(self.pos.x as i32, self.pos.y as i32, 20, 20);
        let _ = match renderer.fill_rect(&ball) {
            Ok(_)    => {},
            Err(err) => fail!("failed to draw rect: {}", err) 
        };
    }
}

impl Ball {
    fn update(& mut self, timestep: uint, others: Vec<Player>) {
        // TODO: Hardcoded width is kinda ugly
        // TODO: Shouldn't bounce off the scoring wall
        if (self.pos.x + 20.0 >= 640.0 && self.mov.dx > 0.0 ) || 
           (self.pos.x <= 0.0 && self.mov.dx < 0.0) {
            self.mov.dx = 0.0;
            self.mov.dy = 0.0;
            return;
        }

        for other in others.iter(){
            if ((self.pos.x + 20.0 >= other.pos.x && self.mov.dx > 0.0 ) ||
                (self.pos.x <= 30.0 && self.mov.dx < 0.0)) &&
                self.pos.y > other.pos.y && self.pos.y < other.pos.y + 120.0 {
                    self.mov.dx = -self.mov.dx;
                }
        }

        if (self.pos.y + 20.0 >= 480.0 && self.mov.dy > 0.0) || 
           (self.pos.y <= 0.0 && self.mov.dy < 0.0) {
            self.mov.dy = -self.mov.dy;
        }

        self.pos.x += self.mov.dx * timestep as f32;
        self.pos.y += self.mov.dy * timestep as f32;

    }
}

impl Renderable for Background {
    fn draw(&self, renderer: &sdl2::render::Renderer<Window>) {
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(101, 208, 246));

        // Clear the buffer, using the light blue color set above.
        let _ = renderer.clear();
    }
}

fn main() {
    // initialize our position variables
    let mut background = Background;
    let mut l_bat = Player { 
        pos: Position { x: 0.0, y: 240.0-60.0 },
        mov: Movement { dx: 0.0, dy: 0.0 }
    };
    let mut r_bat = Player {
        pos: Position { x: 640.0-30.0, y: 240.0-60.0 },
        mov: Movement { dx: 0.0, dy: 0.0 }
    };
    let mut ball = Ball { 
        pos: Position { x: 320.0-10.0, y: 240.0-10.0 }, 
        mov: Movement { dx: 0.2, dy: 0.2 },
    };

    
    // start sdl2 with everything
    sdl2::init(sdl2::INIT_EVERYTHING);

    // Create a window
    let window  = match Window::new("Pong Game", PosCentered, PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => fail!("failed to create window: {}", err)
    };

    // Create a rendering context
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => fail!("failed to create renderer: {}", err)
    };

    // loop until we receive a QuitEvent
    let mut last_time = get_ticks();
    let mut current_time = get_ticks();
    let target_frame_time: uint = 1000/100; // 100fps = 10ms
    'event : loop {
        match poll_event() {
            QuitEvent(_) => break 'event,
            // KeyDownEvent(uint, video::Window, KeyCode, ScanCode, Mod),
            KeyDownEvent(timestamp, window, keycode, scancode, modifier)
                         => {
                println!("key pressed: {}", keycode);
                match keycode {
                    sdl2::keycode::UpKey => l_bat.mov.dy = -3.0,
                    sdl2::keycode::DownKey => l_bat.mov.dy = 3.0,
                    _ => continue
                }
            }
            NoEvent      => {
                let step = current_time - last_time;
                l_bat.update(step);
                r_bat.update(step);
                ball.update(step, vec![l_bat, r_bat]);
                background.draw(&renderer);
                l_bat.draw(&renderer);
                r_bat.draw(&renderer);
                ball.draw(&renderer);
                // update_all(&renderer, &mut [
                //                         &mut background as &mut Renderable,
                //                         &mut l_bat as &mut Renderable,
                //                         &mut r_bat as &mut Renderable,
                //                         &mut ball as &mut Renderable
                //                     ], current_time - last_time);
                let _ = renderer.present();
                last_time = current_time;
                current_time = get_ticks();
                 //not sleeping for the whole time gives a smoother result
                let sleep_time = max(0, target_frame_time as int - (current_time as int - last_time as int));
                delay(sleep_time as uint / 2);
            }
        }
    }

    sdl2::quit();
}

