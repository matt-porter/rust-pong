extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::event::{QuitEvent, poll_event};
use sdl2::rect::{Rect};
use sdl2::timer::{delay};

struct Position {
    x: i32,
    y: i32,
}

struct Movement {
    dx: i32,
    dy: i32
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

trait Movable {
    fn update(&self);
}

// TODO: Transfer drawing responsibility to "Drawable" objects
// TODO: define dy / dx in terms of time
// TODO: Some keyboard inputs

impl Renderable for Player {
    fn draw(&self, renderer:  &sdl2::render::Renderer<Window>) {
        // Set the drawing color to a darker blue.
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 153, 204));
        // Create a smaller centered Rect, filling it in the same dark blue.
        let l_bat = Rect::new(self.pos.x, self.pos.y, 30, 120);
        let _ = match renderer.fill_rect(&l_bat) {
            Ok(_)    => {},
            Err(err) => fail!("failed to draw rect: {}", err) 
        };
    }
}

impl Renderable for Ball {
    fn draw(&self, renderer: &sdl2::render::Renderer<Window>) {
        // Set the drawing color to a darker blue.
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 153, 204));
        let ball = Rect::new(self.pos.x, self.pos.y, 20, 20);
        let _ = match renderer.fill_rect(&ball) {
            Ok(_)    => {},
            Err(err) => fail!("failed to draw rect: {}", err) 
        };
    }
}

impl Renderable for Background {
    fn draw(&self, renderer:  &sdl2::render::Renderer<Window>) {
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(101, 208, 246));

        // Clear the buffer, using the light blue color set above.
        let _ = renderer.clear();
    }
}

impl Movable for Player {
    fn update(&self) {
        self.pos.x += self.mov.dx;
        self.pos.y += self.mov.dy
    }
}

impl Movable for Ball {
    fn update(&self) {
        self.pos.x += self.mov.dx;
        self.pos.y += self.mov.dy
    }
}


fn main() {
    // initialize our position variables
    let mut background = Background;
    let mut l_bat = Player { 
        pos: Position { x: 0, y: 240-60 },
        mov: Movement { dx: 0, dy: 0 }
    };
    let mut r_bat = Player {
        pos: Position { x: 640-30, y: 240-60 },
        mov: Movement { dx: 0, dy: 0 }
    };
    let mut ball = Ball { 
        pos: Position { x: 320-10, y: 240-10 }, 
        mov: Movement { dx: 1, dy: 1 },
    };
    let mut renderables: [&Renderable,..4] = [
        &background as &Renderable,
        &l_bat as &Renderable,
        &r_bat as &Renderable,
        &ball as &Renderable
    ];
    let mut movables: [&Movable,..3] = [
        &l_bat as &Movable,
        &r_bat as &Movable,
        &ball as &Movable
    ];

    
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

    for r in renderables.iter() {
        r.draw(&renderer)
    }

    // loop until we receive a QuitEvent
    'event : loop {
        match poll_event() {
            QuitEvent(_) => break 'event,
            _            => {
                for m in movables.iter() {
                    m.update()
                }
                for r in renderables.iter() {
                    r.draw(&renderer)
                }
                let _ = renderer.present();
                delay(16);
            }
        }
    }

    sdl2::quit();
}

