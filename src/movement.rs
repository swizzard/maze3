use crate::Maze;
use rand::{distr::StandardUniform, prelude::*};

pub fn random_step<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
) {
    let mut moved = false;
    while !moved {
        let v: u8 = rng.sample::<u8, StandardUniform>(StandardUniform) % 4;
        moved = match v {
            0 => maze.move_north(),
            1 => maze.move_south(),
            2 => maze.move_east(),
            3 => maze.move_west(),
            _ => panic!("unreachable"),
        }
    }
}
