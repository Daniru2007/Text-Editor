use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    enable_raw_mode()?;
    terminal.clear()?;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        // .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(terminal.size().unwrap());
    loop {
        if let Event::Key(event) = read()? {
            if event.code == KeyCode::Char('q') {
                break;
            }
        }
        terminal.draw(|rect| {
            let block = Block::default().title("Block").borders(Borders::ALL);
            rect.render_widget(block, chunks[0]);

            let text = Text::raw("Hello World!");
            let paragraph = Paragraph::new(text);
            rect.render_widget(paragraph, chunks[1]);
        })?;
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
