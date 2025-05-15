use maze3::*;
use rand::distr::StandardUniform;
use rand::prelude::*;

fn random_step<const N_ROWS: usize, const N_COLS: usize>(
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

fn main() {
    let mut m = Maze::<4, 4>::new();
    // m.open_east(BoundedIx2::<3, 3>::new(0, 0).unwrap());
    m.seed_doors_naive();
    // println!("{:?}", m.rooms[m.current_ix]);
    // let m = Maze::<3, 3>::new();
    println!("start\n{}", m.pprint());
    let mut rng = rand::rng();
    let mut steps = 0;
    while m.current_ix != m.goal && steps < 51 {
        random_step(&mut m, &mut rng);
        steps += 1;
        println!("{}", m.pprint())
    }
    println!("done in {steps} steps");
}
