use super::Outcome;
use crate::{
    maze::Maze,
    movement::MazeEvent,
    ui::{self, RoomView, render_maze},
};
use color_eyre::Result;
use crossterm::event;
use multid::iterators::V2Indices;
use rand::rngs::ThreadRng;
use ratatui::{DefaultTerminal, widgets::canvas::Context};
pub fn game<const N_ROWS: usize, const N_COLS: usize>(
    mut terminal: DefaultTerminal,
    maze: &mut Maze<N_ROWS, N_COLS>,
    _rng: &mut ThreadRng,
) -> Result<Outcome> {
    loop {
        let render = render_maze_basic(maze);
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

fn render_maze_basic<const N_ROWS: usize, const N_COLS: usize>(
    maze: &Maze<N_ROWS, N_COLS>,
) -> impl Fn(&mut Context) {
    move |ctx| {
        for ix in V2Indices::<N_ROWS, N_COLS>::new() {
            let room = &maze.rooms[ix];
            let view = RoomView {
                x: -200.0 + ui::ROOM_SIZE * ix.x() as f64,
                y: 200.0 - ui::ROOM_SIZE * ix.y() as f64,
                room,
            };
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
        }
    }
}
