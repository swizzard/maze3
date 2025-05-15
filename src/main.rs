use maze3::*;
use std::io::{Write, stdout};
fn main() {
    let mut m = Maze::<3, 3>::new();
    m.seed_doors_naive();
    // println!("{:?}", m.rooms[m.current_ix]);
    let mut s = String::new();
    m.fprint(&mut s).unwrap();
    let mut st = stdout();
    write!(st, "{s}").unwrap();
}
