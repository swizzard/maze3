use crate::{maze::Maze, movement::MazeEvent};
use color_eyre::Result;
use crossterm::event;
use rand::rngs::ThreadRng;
use ratatui::Frame;

pub mod basic;
pub mod hidden;
pub mod lantern;
pub mod menu;
pub mod seeders;

use menu::{MenuChoice, MenuState};
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

pub fn game_loop<const N_ROWS: usize, const N_COLS: usize>() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut rng = ThreadRng::default();
    let mut menu_state = MenuState::default();
    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_stateful_widget(menu::GameMenu, frame.area(), &mut menu_state)
        })?;
        match menu_state.choice {
            None => (),
            Some(MenuChoice::Quit) => break,
            Some(MenuChoice::Game(Game::Basic)) => {
                let mut maze = new_seeded::<N_ROWS, N_COLS>(&mut rng);
                let outcome = basic::game(&mut terminal, &mut maze)?;
                menu_state.game_over(outcome);
                continue;
            }
            Some(MenuChoice::Game(Game::Hidden)) => {
                let mut maze = new_seeded::<N_ROWS, N_COLS>(&mut rng);
                let outcome = hidden::game(&mut terminal, &mut maze)?;
                menu_state.game_over(outcome);
                continue;
            }
            Some(MenuChoice::Game(Game::Lantern)) => {
                let mut maze = new_seeded::<N_ROWS, N_COLS>(&mut rng);
                let outcome = lantern::game(&mut terminal, &mut maze)?;
                menu_state.game_over(outcome);
                continue;
            }
        };
        menu_state.unchoose();
        match event::read()?.into() {
            MazeEvent::MoveN => &menu_state.select_previous(),
            MazeEvent::MoveS => &menu_state.select_next(),
            MazeEvent::Quit => &menu_state.select_quit(),
            MazeEvent::Enter => &menu_state.choose(),
            _ => &(),
        };
    }
    ratatui::restore();
    Ok(())
}

fn new_seeded<const N_ROWS: usize, const N_COLS: usize>(
    rng: &mut ThreadRng,
) -> Maze<N_ROWS, N_COLS> {
    let mut maze = Maze::<N_ROWS, N_COLS>::default();
    seed_doors_path(&mut maze, rng);
    maze
}
