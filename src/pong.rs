extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::event::{QuitEvent, poll_event};
use sdl2::rect::{Rect};
use sdl2::timer::{delay};

struct Position {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

fn update(pos: Position) -> Position {
    Position { x: pos.x + pos.dx, y: pos.y + pos.dy, dx: pos.dx, dy: pos.dy }
}

fn render(renderer: &sdl2::render::Renderer<Window>, l_bat_pos: Position, r_bat_pos: Position, ball_pos: Position) {
        // Set the drawing csdl2::render::Rendererolor to a light blue.
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(101, 208, 246));

    // Clear the buffer, using the light blue color set above.
    let _ = renderer.clear();

    // Set the drawing color to a darker blue.
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 153, 204));

    // Create centered Rect, draw the outline of the Rect in our dark blue color.
    let ball = Rect::new(ball_pos.x, ball_pos.y, 20, 20);
    let _ = match renderer.fill_rect(&ball) {
        Ok(_)    => {},
        Err(err) => fail!("failed to draw rect: {}", err) 
    };

    // Create a smaller centered Rect, filling it in the same dark blue.
    let l_bat = Rect::new(l_bat_pos.x, l_bat_pos.y, 30, 120);
    let _ = match renderer.fill_rect(&l_bat) {
        Ok(_)    => {},
        Err(err) => fail!("failed to draw rect: {}", err) 
    };

    // Create a smaller centered Rect, filling it in the same dark blue.
    let r_bat = Rect::new(r_bat_pos.x, r_bat_pos.y, 30, 120);
    let _ = match renderer.fill_rect(&r_bat) {
        Ok(_)    => {},
        Err(err) => fail!("failed to draw rect: {}", err) 
    };

    // Swap our buffer for the present buffer, displaying it.
    let _ = renderer.present();
}

fn main() {
    // initialize our position variables
    let mut l_bat_pos = Position { x: 0, y: 240-60, dx: 0, dy: 0 };
    let mut r_bat_pos = Position { x: 640-30, y: 240-60, dx: 0, dy: 0 };
    let mut ball_pos  = Position { x: 320-10, y: 240-10, dx: 1, dy: 1 };

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

    render(&renderer, l_bat_pos, r_bat_pos, ball_pos);

    // loop until we receive a QuitEvent
    'event : loop {
        match poll_event() {
            QuitEvent(_) => break 'event,
            _            => {
                l_bat_pos = update(l_bat_pos);
                r_bat_pos = update(r_bat_pos);
                ball_pos  = update(ball_pos);
                render(&renderer, l_bat_pos, r_bat_pos, ball_pos);
                delay(16);
            }
        }
    }

    sdl2::quit();
}

