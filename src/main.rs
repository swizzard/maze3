use color_eyre::Result;
use maze3::*;
use rand::distr::StandardUniform;
use rand::prelude::*;

fn main() {
    let mut m = Maze::<4, 4>::new();
    let mut rng = rand::rng();
    m.seed_doors_naive(&mut rng);
    println!("start\n{}", m.pprint());
    let mut steps = 0;
    while m.current_ix != m.goal && steps < 51 {
        maze3::movement::random_step(&mut m, &mut rng);
        steps += 1;
        println!("{}", m.pprint())
    }
    println!("done in {steps} steps");
}
