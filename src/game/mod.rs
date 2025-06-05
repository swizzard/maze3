use crate::{maze::Maze, movement::MazeEvent};
use color_eyre::Result;
use crossterm::event;
use rand::rngs::ThreadRng;
use ratatui::{Frame, widgets::ListState};

pub mod basic;
pub mod hidden;
pub mod lantern;
pub mod menu;
pub mod seeders;

use menu::MenuChoice;
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
    let mut choice: Option<MenuChoice> = None;
    let mut menu_state = ListState::default();
    menu_state.select_first();
    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_stateful_widget(menu::GameMenu, frame.area(), &mut menu_state)
        })?;
        match choice {
            None => (),
            Some(MenuChoice::Quit) => break,
            Some(MenuChoice::Game(Game::Basic)) => {
                let mut maze = new_seeded::<N_ROWS, N_COLS>(&mut rng);
                basic::game(&mut terminal, &mut maze)?;
                choice = None;
                continue;
            }
            Some(MenuChoice::Game(Game::Hidden)) => {
                let mut maze = new_seeded::<N_ROWS, N_COLS>(&mut rng);
                hidden::game(&mut terminal, &mut maze)?;
                choice = None;
                continue;
            }
            Some(MenuChoice::Game(Game::Lantern)) => {
                let mut maze = new_seeded::<N_ROWS, N_COLS>(&mut rng);
                lantern::game(&mut terminal, &mut maze)?;
                choice = None;
                continue;
            }
        };
        choice = None;
        match event::read()?.into() {
            MazeEvent::MoveN => &menu_state.select_previous(),
            MazeEvent::MoveS => &menu_state.select_next(),
            MazeEvent::Quit => &menu_state.select_last(),
            MazeEvent::Enter => {
                choice = menu_state.selected().map(MenuChoice::from);
                &()
            }
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

// pub fn render_ui<const N_ROWS: usize, const N_COLS: usize>(
//     maze: &mut Maze<N_ROWS, N_COLS>,
//     rng: &mut ThreadRng,
//     game: Game,
// ) -> Result<()> {
//     let terminal = ratatui::init();
//     let game_fn = match game {
//         Game::Basic => basic::game,
//         Game::Hidden => hidden::game,
//         Game::Lantern => lantern::game,
//     };
//     let result = game_fn(terminal, maze, rng);
//     ratatui::restore();
//     result.map(|outcome| match outcome {
//         Outcome::Win => println!("You won!"),
//         Outcome::Quit => println!("You quit!"),
//     })
// }
