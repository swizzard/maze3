use color_eyre::Result;
use samazing::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    game_loop::<7, 7>()
}
