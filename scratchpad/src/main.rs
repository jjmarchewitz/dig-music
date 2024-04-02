mod scripts;

use eyre::Result;

fn main() -> Result<()> {
    let playlist_str = scripts::top_intersecting::run()?;

    println!("{}", playlist_str);

    Ok(())
}
