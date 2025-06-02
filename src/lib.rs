#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod game;
pub mod maze;
pub mod movement;
pub mod ui;

pub use game::{Game, render_ui, seed_doors_naive};
pub use maze::Maze;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
