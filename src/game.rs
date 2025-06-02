use crate::{
    maze::{DoorState, Maze},
    movement::MazeEvent,
    ui::{self, RoomView},
};
use color_eyre::Result;
use crossterm::event;
use multid::{BoundedIx2, iterators::V2Indices};
use rand::{Rng, rngs::ThreadRng};
use ratatui::{DefaultTerminal, Frame, widgets::canvas::Canvas};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Game {
    Basic,
}

pub fn render_ui<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
    game: Game,
) -> Result<()> {
    let terminal = ratatui::init();
    let game_fn = match game {
        Game::Basic => basic,
    };
    let result = game_fn(terminal, maze, rng);
    ratatui::restore();
    result
}
fn basic<const N_ROWS: usize, const N_COLS: usize>(
    mut terminal: DefaultTerminal,
    maze: &mut Maze<N_ROWS, N_COLS>,
    _rng: &mut ThreadRng,
) -> Result<()> {
    loop {
        let render = render_maze_basic(maze);
        terminal.draw(render)?;
        match event::read()?.into() {
            MazeEvent::MoveN => maze.move_north(),
            MazeEvent::MoveS => maze.move_south(),
            MazeEvent::MoveE => maze.move_east(),
            MazeEvent::MoveW => maze.move_west(),
            _ => break,
        };
    }
    Ok(())
}

fn render_maze_basic<const N_ROWS: usize, const N_COLS: usize>(
    maze: &Maze<N_ROWS, N_COLS>,
) -> impl FnOnce(&mut Frame) {
    let widget = Canvas::default()
        .x_bounds([-200.0, 200.0])
        .y_bounds([-200.0, 200.0])
        .background_color(ui::BG_COLOR)
        .paint(move |ctx| {
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
        });
    |f: &mut Frame| f.render_widget(widget, f.area())
}

pub fn seed_doors_naive<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
) {
    for ix in V2Indices::<N_ROWS, N_COLS>::new() {
        if rng.random_bool(0.5) {
            maze.open_north(ix);
        }
        if rng.random_bool(0.5) {
            maze.open_south(ix);
        }
        if rng.random_bool(0.5) {
            maze.open_east(ix);
        }
        if rng.random_bool(0.5) {
            maze.open_west(ix);
        }
    }
    // ensure we can get out of starting room
    let start_ix = BoundedIx2::min();
    if maze.rooms[start_ix].doors.east == Some(DoorState::Closed)
        && maze.rooms[start_ix].doors.south == Some(DoorState::Closed)
    {
        if rng.random_bool(0.5) {
            maze.open_west(start_ix);
        } else {
            maze.open_south(start_ix);
        }
    }
    // ensure we can get into end room
    let end_ix = BoundedIx2::max();
    if maze.rooms[end_ix].doors.west == Some(DoorState::Closed)
        && maze.rooms[end_ix].doors.north == Some(DoorState::Closed)
    {
        if rng.random_bool(0.5) {
            maze.open_east(end_ix);
        } else {
            maze.open_north(end_ix);
        }
    }
}
