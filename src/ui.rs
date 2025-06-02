use crate::maze::{DoorState, Room};
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
pub const DOOR_COLOR: Color = Color::Red;

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

pub fn render_maze<'a, const N_ROWS: usize, const N_COLS: usize, F>(
    f: F,
) -> impl FnOnce(&'a mut Frame)
where
    F: Fn(&mut Context),
{
    let widget = Canvas::default()
        .x_bounds([-200.0, 200.0])
        .y_bounds([-200.0, 200.0])
        .background_color(BG_COLOR)
        .paint(f);
    |frame: &mut Frame| frame.render_widget(widget, frame.area())
}
