use super::Outcome;
use crate::{
    maze::Maze,
    movement::MazeEvent,
    ui::{self, RoomView},
};
use color_eyre::Result;
use crossterm::event;
use multid::iterators::V2Indices;
use rand::rngs::ThreadRng;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget, canvas::Canvas},
};

pub struct BasicGame<const N_ROWS: usize, const N_COLS: usize>;

impl<const N_ROWS: usize, const N_COLS: usize> StatefulWidget for BasicGame<N_ROWS, N_COLS> {
    type State = Maze<N_ROWS, N_COLS>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let c = Canvas::default()
            .x_bounds([ui::MIN_X, ui::MAX_X])
            .y_bounds([ui::MIN_Y, ui::MAX_Y])
            .background_color(ui::BG_COLOR)
            .paint(move |ctx| {
                for ix in V2Indices::<N_ROWS, N_COLS>::new() {
                    let room = &state.rooms[ix];
                    let view = RoomView {
                        x: -200.0 + ui::ROOM_SIZE * ix.x() as f64,
                        y: 200.0 - ui::ROOM_SIZE * ix.y() as f64,
                        room,
                    };
                    ctx.draw(&view);
                    let label_x = -200.0 + (ui::ROOM_SIZE * ix.x() as f64) + ui::SEG_LEN * 3.5;
                    let label_y = 200.0 - (ui::ROOM_SIZE * ix.y() as f64 + ui::SEG_LEN * 3.5);
                    if ix == state.current_ix && ix == state.goal {
                        ctx.print(label_x, label_y, "\u{1f940}")
                    } else if ix == state.current_ix {
                        ctx.print(label_x, label_y, "\u{1f600}")
                    } else if ix == state.goal {
                        ctx.print(label_x, label_y, "\u{1f945}")
                    }
                }
            });
        Widget::render(c, area, buf);
    }
}

pub fn game<const N_ROWS: usize, const N_COLS: usize>(
    mut terminal: DefaultTerminal,
    maze: &mut Maze<N_ROWS, N_COLS>,
    _rng: &mut ThreadRng,
) -> Result<Outcome> {
    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_stateful_widget(BasicGame {}, frame.area(), maze)
        })?;
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
