mod game;
mod geometry;

extern crate piston_window;
use piston_window::*;

fn main() {
    let rect = geometry::Rect::new(0, 0, 20, 20);
    let border = geometry::Border::new(1, 1, 1, 1);
    let mut game_board = game::Board::new(rect, border);
    let size = [
        (rect.x_max * game::draw::SCALE) as u32,
        (rect.y_max * game::draw::SCALE) as u32,
    ];
    let mut window: PistonWindow = WindowSettings::new("Snake", size).exit_on_esc(true).build().unwrap();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game_board.key_press(key);
        }

        window.draw_2d(&e, |c, g, _| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            game_board.draw(&c, g);
        });

        e.update(|arg| {
            game_board.update(arg.dt);
        });
    }
}
