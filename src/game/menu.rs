use super::{Game, Outcome};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListState, Padding, Paragraph, StatefulWidget, Widget},
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

#[derive(Debug)]
pub struct MenuState {
    list: ListState,
    pub choice: Option<MenuChoice>,
    prev_outcome: Option<Outcome>,
}

impl MenuState {
    pub fn game_over(&mut self, outcome: Outcome) {
        self.choice = None;
        self.prev_outcome = Some(outcome);
        self.list.select_first();
    }
    pub fn unchoose(&mut self) {
        self.choice = None;
    }
    pub fn choose(&mut self) {
        self.choice = self.list.selected().map(MenuChoice::from)
    }
    pub fn select_previous(&mut self) {
        self.list.select_previous();
    }
    pub fn select_next(&mut self) {
        self.list.select_next();
    }
    pub fn select_quit(&mut self) {
        self.list.select_last();
    }
    pub fn outcome_msg(&self) -> &str {
        match self.prev_outcome {
            None => "",
            Some(Outcome::Win) => "you won!",
            Some(Outcome::Quit) => "you quit",
        }
    }
    fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list
    }
}

impl Default for MenuState {
    fn default() -> Self {
        let mut this = MenuState {
            list: ListState::default(),
            choice: None,
            prev_outcome: None,
        };
        this.list.select_first();
        this
    }
}

impl StatefulWidget for GameMenu {
    type State = MenuState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let b = Block::bordered()
            .title_alignment(Alignment::Center)
            .title("Menu")
            .fg(Color::Green)
            .padding(Padding::symmetric(5, 1));
        let inner_area = b.inner(area);
        let vertical = Layout::vertical([Constraint::Min(0), Constraint::Length(5)]);
        let [menu_area, outcome_area] = vertical.areas(inner_area);
        let l = MenuChoice::to_list()
            .block(Block::bordered())
            .fg(Color::Green)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("*");
        Widget::render(b, area, buf);
        StatefulWidget::render(l, menu_area, buf, state.list_state_mut());
        Widget::render(
            Paragraph::new(state.outcome_msg())
                .alignment(Alignment::Center)
                .block(Block::bordered())
                .fg(Color::Green),
            outcome_area,
            buf,
        );
    }
}
