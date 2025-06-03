#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod game;
pub mod maze;
pub mod movement;
pub mod ui;

pub use game::{Game, render_ui, seed_doors_path};
pub use maze::Maze;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl IntoIterator for Direction {
    type Item = Self;
    type IntoIter = DirectionsIter;

    fn into_iter(self) -> Self::IntoIter {
        DirectionsIter::new()
    }
}

pub struct DirectionsIter {
    curr: Option<Direction>,
}

impl DirectionsIter {
    fn new() -> Self {
        Self { curr: None }
    }
}

impl Iterator for DirectionsIter {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            None => {
                let dir = Some(Direction::North);
                self.curr = dir;
                dir
            }
            Some(Direction::North) => {
                let dir = Some(Direction::East);
                self.curr = dir;
                dir
            }
            Some(Direction::East) => {
                let dir = Some(Direction::South);
                self.curr = dir;
                dir
            }
            Some(Direction::South) => {
                let dir = Some(Direction::West);
                self.curr = dir;
                dir
            }
            Some(Direction::West) => {
                let dir = None;
                self.curr = dir;
                dir
            }
        }
    }
}
