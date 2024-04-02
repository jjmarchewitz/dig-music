mod scripts;

use arboard::Clipboard;
use eyre::Result;

fn main() -> Result<()> {
    let playlist_str = scripts::top_intersecting::run()?;

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(playlist_str)?;

    println!("\n> Copied to clipboard!");

    Ok(())
}
