mod scripts;

use arboard::Clipboard;
use eyre::Result;

fn main() -> Result<()> {
    // let output_str = scripts::top_intersecting::run()?;
    let output_str = scripts::find_song::run()?;

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(output_str)?;

    println!("\n> Copied to clipboard!");

    Ok(())
}
