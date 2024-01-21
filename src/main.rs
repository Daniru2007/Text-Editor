use crossterm::terminal::enable_raw_mode;
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            rect.render_widget(block, size);
        });
        thread::sleep(Duration::from_millis(5000));
    }
    Ok(())
}
