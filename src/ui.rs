use crate::{DoorState, Maze, Room, movement::random_step};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use multid::iterators::V2Indices;
use rand::rngs::ThreadRng;
use ratatui::{
    DefaultTerminal, Frame,
    style::Color,
    widgets::canvas::{Canvas, Line, Painter, Shape},
};

const SEG_LEN: f64 = 8.0;
const SEG_COUNT: f64 = 7.0;
const ROOM_SIZE: f64 = SEG_LEN * SEG_COUNT;
const BG_COLOR: Color = Color::Black;
const WALL_COLOR: Color = Color::Green;
const DOOR_COLOR: Color = Color::Red;

#[derive(Debug)]
pub struct RoomView<'a> {
    pub x: f64,
    pub y: f64,
    pub room: &'a Room,
}

impl<'a> Shape for RoomView<'a> {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let lines: &[Line] = &[
            // north
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x + SEG_LEN * 2.0,
                y2: self.y,
                color: WALL_COLOR,
            },
            Line {
                x1: self.x + SEG_LEN * 2.0,
                y1: self.y,
                x2: self.x + SEG_LEN * 5.0,
                y2: self.y,
                color: door_state_color(&self.room.doors.north),
            },
            Line {
                x1: self.x + SEG_LEN * 5.0,
                y1: self.y,
                x2: self.x + SEG_LEN * 7.0,
                y2: self.y,
                color: WALL_COLOR,
            },
            // west
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x,
                y2: self.y - SEG_LEN * 3.0,
                color: WALL_COLOR,
            },
            Line {
                x1: self.x,
                y1: self.y - SEG_LEN * 3.0,
                x2: self.x,
                y2: self.y - SEG_LEN * 5.0,
                color: door_state_color(&self.room.doors.west),
            },
            Line {
                x1: self.x,
                y1: self.y - SEG_LEN * 5.0,
                x2: self.x,
                y2: self.y - SEG_LEN * 7.0,
                color: WALL_COLOR,
            },
            // south
            Line {
                x1: self.x,
                y1: self.y - SEG_LEN * 7.0,
                x2: self.x + SEG_LEN * 2.0,
                y2: self.y - SEG_LEN * 7.0,
                color: WALL_COLOR,
            },
            Line {
                x1: self.x + SEG_LEN * 2.0,
                y1: self.y - SEG_LEN * 7.0,
                x2: self.x + SEG_LEN * 5.0,
                y2: self.y - SEG_LEN * 7.0,
                color: door_state_color(&self.room.doors.south),
            },
            Line {
                x1: self.x + SEG_LEN * 5.0,
                y1: self.y - SEG_LEN * 7.0,
                x2: self.x + SEG_LEN * 7.0,
                y2: self.y - SEG_LEN * 7.0,
                color: WALL_COLOR,
            },
            // east
            Line {
                x1: self.x + SEG_LEN * 7.0,
                y1: self.y,
                x2: self.x + SEG_LEN * 7.0,
                y2: self.y - SEG_LEN * 3.0,
                color: WALL_COLOR,
            },
            Line {
                x1: self.x + SEG_LEN * 7.0,
                y1: self.y - SEG_LEN * 3.0,
                x2: self.x + SEG_LEN * 7.0,
                y2: self.y - SEG_LEN * 5.0,
                color: door_state_color(&self.room.doors.east),
            },
            Line {
                x1: self.x + SEG_LEN * 7.0,
                y1: self.y - SEG_LEN * 5.0,
                x2: self.x + SEG_LEN * 7.0,
                y2: self.y - SEG_LEN * 7.0,
                color: WALL_COLOR,
            },
        ];
        for line in lines {
            line.draw(painter)
        }
    }
}

fn door_state_color(ds: &Option<DoorState>) -> Color {
    match ds {
        None => WALL_COLOR,
        Some(DoorState::Open) => BG_COLOR,
        Some(DoorState::Closed) => DOOR_COLOR,
    }
}

pub fn render_ui<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
) -> Result<()> {
    let terminal = ratatui::init();
    let result = ui_loop(terminal, maze, rng);
    ratatui::restore();
    result
}
fn ui_loop<const N_ROWS: usize, const N_COLS: usize>(
    mut terminal: DefaultTerminal,
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
) -> Result<()> {
    loop {
        let render = render_maze(maze);
        terminal.draw(render)?;
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
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
            }) => {
                maze.move_west();
            }
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
            }) => {
                maze.move_east();
            }
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
            }) => {
                maze.move_north();
            }
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
            }) => {
                maze.move_south();
            }
            _ if !maze.is_done() => {
                random_step(maze, rng);
            }
            _ => break,
        }
    }
    Ok(())
}

fn render_maze<const N_ROWS: usize, const N_COLS: usize>(
    maze: &Maze<N_ROWS, N_COLS>,
) -> impl FnOnce(&mut Frame) {
    let widget = Canvas::default()
        .x_bounds([-200.0, 200.0])
        .y_bounds([-200.0, 200.0])
        .background_color(BG_COLOR)
        .paint(move |ctx| {
            for ix in V2Indices::<N_ROWS, N_COLS>::new() {
                let room = &maze.rooms[ix];
                let view = RoomView {
                    x: -200.0 + ROOM_SIZE * ix.x() as f64,
                    y: 200.0 - ROOM_SIZE * ix.y() as f64,
                    room,
                };
                ctx.draw(&view);
                let label_x = -200.0 + (ROOM_SIZE * ix.x() as f64) + SEG_LEN * 3.5;
                let label_y = 200.0 - (ROOM_SIZE * ix.y() as f64 + SEG_LEN * 3.5);
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
