use crate::maze::Maze;
use crossterm::event::{Event, KeyCode, KeyEvent};
use rand::{distr::StandardUniform, prelude::*};

pub fn random_step<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
) {
    let mut moved = false;
    while !moved {
        let v: u8 = rng.sample::<u8, StandardUniform>(StandardUniform) % 4;
        moved = match v {
            0 => maze.move_north(),
            1 => maze.move_south(),
            2 => maze.move_east(),
            3 => maze.move_west(),
            _ => panic!("unreachable"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MazeEvent {
    MoveN,
    MoveS,
    MoveE,
    MoveW,
    Enter,
    Quit,
    OtherKey(KeyCode),
    Other(Event),
}

impl From<Event> for MazeEvent {
    fn from(val: Event) -> Self {
        match val {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => MazeEvent::Quit,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                ..
            }) => MazeEvent::MoveW,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('l'),
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                ..
            }) => MazeEvent::MoveE,
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('k'),
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                ..
            }) => MazeEvent::MoveN,
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                ..
            }) => MazeEvent::MoveS,
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                ..
            }) => MazeEvent::Enter,
            Event::Key(KeyEvent { code: kc, .. }) => MazeEvent::OtherKey(kc),
            other => MazeEvent::Other(other),
        }
    }
}
