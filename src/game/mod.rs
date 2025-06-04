use crate::maze::Maze;
use color_eyre::Result;
use rand::rngs::ThreadRng;

pub mod basic;
pub mod hidden;
pub mod lantern;
pub mod menu;
pub mod seeders;

pub use seeders::{seed_doors_naive, seed_doors_path};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Game {
    Basic,
    Hidden,
    Lantern,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    Win,
    Quit,
}

pub fn render_ui<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
    game: Game,
) -> Result<()> {
    let terminal = ratatui::init();
    let game_fn = match game {
        Game::Basic => basic::game,
        Game::Hidden => hidden::game,
        Game::Lantern => lantern::game,
    };
    let result = game_fn(terminal, maze, rng);
    ratatui::restore();
    result.map(|outcome| match outcome {
        Outcome::Win => println!("You won!"),
        Outcome::Quit => println!("You quit!"),
    })
}
