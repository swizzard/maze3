use color_eyre::Result;
use maze3::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut m = Maze::<7, 7>::new();
    let mut rng = rand::rng();
    seed_doors_path(&mut m, &mut rng);
    render_ui(&mut m, &mut rng, Game::Basic)
}
