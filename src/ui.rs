use crate::{
    Direction,
    maze::{DoorState, Room},
};
use ratatui::{
    Frame,
    style::Color,
    widgets::canvas::{Canvas, Context, Line, Painter, Shape},
};

pub const MIN_X: f64 = -200.0;
pub const MAX_X: f64 = 200.0;
pub const MIN_Y: f64 = -200.0;
pub const MAX_Y: f64 = 200.0;
pub const SEG_LEN: f64 = 8.0;
pub const SEG_COUNT: f64 = 7.0;
pub const ROOM_SIZE: f64 = SEG_LEN * SEG_COUNT;
pub const BG_COLOR: Color = Color::Black;
pub const WALL_COLOR: Color = Color::Green;
pub const HIDDEN_WALL_COLOR: Color = Color::Gray;
pub const DOOR_COLOR: Color = Color::Red;
pub fn render_maze<const N_ROWS: usize, const N_COLS: usize, F>(
    f: F,
) -> impl for<'a> FnOnce(&'a mut Frame)
where
    F: Fn(&mut Context),
{
    let widget = Canvas::default()
        .x_bounds([MIN_X, MAX_X])
        .y_bounds([MIN_Y, MAX_Y])
        .background_color(BG_COLOR)
        .paint(f);
    |frame: &mut Frame| frame.render_widget(widget, frame.area())
}

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

#[derive(Debug)]
pub struct UnseenRoomView {
    pub x: f64,
    pub y: f64,
    pub hidden_walls: Vec<Direction>,
}

impl UnseenRoomView {
    fn draw_north_line(&self, painter: &mut Painter<'_, '_>, color: Color) {
        Line {
            x1: self.x,
            y1: self.y,
            x2: self.x + ROOM_SIZE,
            y2: self.y,
            color,
        }
        .draw(painter);
    }
    fn draw_west_line(&self, painter: &mut Painter<'_, '_>, color: Color) {
        Line {
            x1: self.x,
            y1: self.y,
            x2: self.x,
            y2: self.y - ROOM_SIZE,
            color,
        }
        .draw(painter);
    }
    fn draw_south_line(&self, painter: &mut Painter<'_, '_>, color: Color) {
        Line {
            x1: self.x,
            y1: self.y - ROOM_SIZE,
            x2: self.x + ROOM_SIZE,
            y2: self.y - ROOM_SIZE,
            color,
        }
        .draw(painter);
    }
    fn draw_east_line(&self, painter: &mut Painter<'_, '_>, color: Color) {
        Line {
            x1: self.x + ROOM_SIZE,
            y1: self.y,
            x2: self.x + ROOM_SIZE,
            y2: self.y - ROOM_SIZE,
            color,
        }
        .draw(painter)
    }
}

impl Shape for UnseenRoomView {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for wall in self.hidden_walls.iter() {
            match wall {
                Direction::North => {
                    self.draw_north_line(painter, HIDDEN_WALL_COLOR);
                }
                Direction::West => {
                    self.draw_west_line(painter, HIDDEN_WALL_COLOR);
                }
                Direction::South => {
                    self.draw_south_line(painter, HIDDEN_WALL_COLOR);
                }
                Direction::East => {
                    self.draw_east_line(painter, HIDDEN_WALL_COLOR);
                }
            }
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
