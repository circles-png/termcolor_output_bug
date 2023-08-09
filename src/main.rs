use anyhow::{Result, Ok};
use termcolor::{ColorChoice, StandardStream, Color};
use termcolor_output::colored;

fn main() -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    colored!(stdout, "{}hello, {}!", fg!(Some(Color::Green)), "world").unwrap();
    Ok(())
}
