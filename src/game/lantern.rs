use super::Outcome;
use crate::{
    Direction,
    maze::Maze,
    movement::MazeEvent,
    ui::{self, RoomView, UnseenRoomView, render_maze},
};
use color_eyre::Result;
use crossterm::event;
use multid::{BoundedIx2, iterators::Ix2Neighbors};
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
        let render = render_maze_lantern(maze, &seen);
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

fn render_maze_lantern<const N_ROWS: usize, const N_COLS: usize>(
    maze: &Maze<N_ROWS, N_COLS>,
    seen: &BTreeSet<BoundedIx2<N_ROWS, N_COLS>>,
) -> impl Fn(&mut Context) {
    move |ctx| {
        let curr_ix = maze.current_ix;
        // let ns = Ix2Neighbors::<N_ROWS, N_COLS>::new(maze.current_ix)
        //     .chain(std::iter::once(curr_ix))
        //     .collect::<Vec<BoundedIx2<N_ROWS, N_COLS>>>();
        // ctx.print(-200.0, -200.0, format!("{ns:?}"));
        for ix in
            Ix2Neighbors::<N_ROWS, N_COLS>::new(maze.current_ix).chain(std::iter::once(curr_ix))
        {
            let x = -70.0 + ui::ROOM_SIZE * signed_diff(ix.x(), curr_ix.x());
            let y = 30.0 - ui::ROOM_SIZE * signed_diff(ix.y(), curr_ix.y());
            if true {
                // if seen.contains(&ix) {
                let room = &maze.rooms[ix];
                let view = RoomView { x, y, room };
                ctx.draw(&view);
                let label_x = x + (ui::SEG_LEN * 3.0);
                let label_y = y - (ui::SEG_LEN * 4.0);
                // ctx.print(label_x - 20.0, label_y - 20.0, format!("{ix:?}"));
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

fn signed_diff(a: usize, b: usize) -> f64 {
    let mut res = a.abs_diff(b) as f64;
    if b > a {
        res *= -1.0;
    }
    res
}
