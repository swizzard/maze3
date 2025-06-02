use color_eyre::Result;
use maze3::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut m = Maze::<7, 7>::new();
    let mut rng = rand::rng();
    m.seed_doors_naive(&mut rng);
    maze3::ui::render_ui(&mut m, &mut rng)
}
