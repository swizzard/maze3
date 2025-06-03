use crate::{Direction, maze::Maze};
use multid::{BoundedIx2, iterators::V2Indices};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};
use std::collections::BTreeSet;
pub fn seed_doors_naive<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
) {
    for ix in V2Indices::<N_ROWS, N_COLS>::new() {
        while !maze.rooms[ix].doors.any_open() {
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
    }
}

pub fn seed_doors_path<const N_ROWS: usize, const N_COLS: usize>(
    maze: &mut Maze<N_ROWS, N_COLS>,
    rng: &mut ThreadRng,
) {
    let mut all_visited: BTreeSet<BoundedIx2<N_ROWS, N_COLS>> = BTreeSet::new();
    'outer: loop {
        let mut visited: BTreeSet<BoundedIx2<N_ROWS, N_COLS>> = BTreeSet::new();
        let mut curr: BoundedIx2<N_ROWS, N_COLS> = maze.current_ix;
        loop {
            if curr == maze.goal {
                break 'outer;
            }
            visited.insert(curr);
            let available: Vec<Direction> = maze.rooms[curr]
                .all_doors()
                .filter_map::<Direction, _>(|(dir, _)| match dir {
                    Direction::North => curr.north().and_then(|ix| {
                        if visited.contains(&ix) {
                            None
                        } else {
                            Some(dir)
                        }
                    }),
                    Direction::East => curr.east().and_then(|ix| {
                        if visited.contains(&ix) {
                            None
                        } else {
                            Some(dir)
                        }
                    }),
                    Direction::South => curr.south().and_then(|ix| {
                        if visited.contains(&ix) {
                            None
                        } else {
                            Some(dir)
                        }
                    }),
                    Direction::West => curr.west().and_then(|ix| {
                        if visited.contains(&ix) {
                            None
                        } else {
                            Some(dir)
                        }
                    }),
                })
                .collect();
            match available.choose(rng) {
                None => {
                    all_visited.append(&mut visited);
                    break;
                }
                Some(Direction::North) => {
                    maze.open_north(curr);
                    curr = curr.north().unwrap()
                }
                Some(Direction::East) => {
                    maze.open_east(curr);
                    curr = curr.east().unwrap()
                }
                Some(Direction::South) => {
                    maze.open_south(curr);
                    curr = curr.south().unwrap()
                }
                Some(Direction::West) => {
                    maze.open_west(curr);
                    curr = curr.west().unwrap()
                }
            }
        }
    }
    for ix in V2Indices::<N_ROWS, N_COLS>::new() {
        if !all_visited.contains(&ix) {
            match maze.rooms[ix]
                .available_directions()
                .collect::<Vec<Direction>>()
                .choose(rng)
            {
                Some(Direction::North) => maze.open_north(ix),
                Some(Direction::East) => maze.open_east(ix),
                Some(Direction::South) => maze.open_south(ix),
                Some(Direction::West) => maze.open_west(ix),
                None => (),
            }
        }
    }
}
