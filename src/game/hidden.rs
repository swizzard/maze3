use super::Outcome;
use crate::{
    Direction,
    maze::Maze,
    movement::MazeEvent,
    ui::{self, RoomView, UnseenRoomView, render_maze},
};
use color_eyre::Result;
use crossterm::event;
use multid::{BoundedIx2, iterators::V2Indices};
use rand::rngs::ThreadRng;
use ratatui::{DefaultTerminal, widgets::canvas::Context};
use std::collections::BTreeSet;
pub fn game<const N_ROWS: usize, const N_COLS: usize>(
    mut terminal: DefaultTerminal,
    maze: &mut Maze<N_ROWS, N_COLS>,
    _rng: &mut ThreadRng,
) -> Result<Outcome> {
    let mut seen: BTreeSet<BoundedIx2<N_ROWS, N_COLS>> = BTreeSet::new();
    loop {
        seen.insert(maze.current_ix);
        let render = render_maze_hidden(maze, &seen);
        let render = render_maze::<N_ROWS, N_COLS, _>(render);
        terminal.draw(render)?;
        if maze.is_done() {
            return Ok(Outcome::Win);
        }
        match event::read()?.into() {
            MazeEvent::MoveN => maze.move_north(),
            MazeEvent::MoveS => maze.move_south(),
            MazeEvent::MoveE => maze.move_east(),
            MazeEvent::MoveW => maze.move_west(),
            MazeEvent::Quit => return Ok(Outcome::Quit),
            _ => false,
        };
    }
}

fn render_maze_hidden<const N_ROWS: usize, const N_COLS: usize>(
    maze: &Maze<N_ROWS, N_COLS>,
    seen: &BTreeSet<BoundedIx2<N_ROWS, N_COLS>>,
) -> impl Fn(&mut Context) {
    move |ctx| {
        for ix in V2Indices::<N_ROWS, N_COLS>::new() {
            let x = -200.0 + ui::ROOM_SIZE * ix.x() as f64;
            let y = 200.0 - ui::ROOM_SIZE * ix.y() as f64;
            if seen.contains(&ix) {
                let room = &maze.rooms[ix];
                let view = RoomView { x, y, room };
                ctx.draw(&view);
                let label_x = -200.0 + (ui::ROOM_SIZE * ix.x() as f64) + ui::SEG_LEN * 3.5;
                let label_y = 200.0 - (ui::ROOM_SIZE * ix.y() as f64 + ui::SEG_LEN * 3.5);
                if ix == maze.current_ix && ix == maze.goal {
                    ctx.print(label_x, label_y, "\u{1f940}")
                } else if ix == maze.current_ix {
                    ctx.print(label_x, label_y, "\u{1f600}")
                } else if ix == maze.goal {
                    ctx.print(label_x, label_y, "\u{1f945}")
                }
            } else {
                let mut unseen: Vec<Direction> = Vec::with_capacity(4);
                if ix.north().map(|i| !seen.contains(&i)).unwrap_or(true) {
                    unseen.push(Direction::North);
                }
                if ix.south().map(|i| !seen.contains(&i)).unwrap_or(true) {
                    unseen.push(Direction::South);
                }
                if ix.east().map(|i| !seen.contains(&i)).unwrap_or(true) {
                    unseen.push(Direction::East);
                }
                if ix.west().map(|i| !seen.contains(&i)).unwrap_or(true) {
                    unseen.push(Direction::West);
                }
                ctx.draw(&UnseenRoomView {
                    x,
                    y,
                    hidden_walls: unseen,
                });
            }
        }
    }
}
