extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::event::{QuitEvent, poll_event};
use sdl2::rect::{Rect};

struct Position {
    x: i32,
    y: i32
}

fn main() {
    // initialize our position variables
    let l_bat_pos = Position { x: 0, y: 240-60 };
    let r_bat_pos = Position { x: 640-30, y: 240-60 };
    let ball_pos  = Position { x: 320-10, y: 240-10 };

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

    // Set the drawing color to a light blue.
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

    // loop until we receive a QuitEvent
    'event : loop {
        match poll_event() {
            QuitEvent(_) => break 'event,
            _            => continue
        }
    }

    sdl2::quit();
}

