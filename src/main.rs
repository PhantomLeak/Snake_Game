mod snake;
mod direction;
mod game;
mod point;
mod command;

use crate::game::Game;
use std::io::stdout;
fn main() {
    Game::new(stdout(), 15, 15).run();
}