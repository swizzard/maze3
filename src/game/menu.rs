use super::Game;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListState, StatefulWidget},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MenuChoice {
    Quit,
    Game(Game),
}

impl MenuChoice {
    fn to_list<'a>() -> List<'a> {
        List::new(["Basic", "Hidden", "Lantern", "Quit"])
    }
}

impl From<usize> for MenuChoice {
    fn from(val: usize) -> Self {
        match val {
            0 => MenuChoice::Game(Game::Basic),
            1 => MenuChoice::Game(Game::Hidden),
            2 => MenuChoice::Game(Game::Lantern),
            _ => MenuChoice::Quit,
        }
    }
}

pub struct GameMenu;

impl StatefulWidget for GameMenu {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let l = MenuChoice::to_list()
            .block(Block::bordered().title("Menu"))
            .fg(Color::Green)
            .bg(Color::White)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("*");
        StatefulWidget::render(l, area, buf, state)
    }
}
