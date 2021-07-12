use std::io;
use tui::Terminal;
use tui::backend::{CrosstermBackend, Backend};
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};


fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let mut backend = CrosstermBackend::new(stdout);
    backend.clear()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(size);

        let heirarchy = Block::default()
            .title("Accounts")
            .borders(Borders::RIGHT);
        let render = Block::default();

        // Create a bordered block
        let window = Block::default()
            .title("Greymail")
            .borders(Borders::ALL);

        f.render_widget(window, size);
        f.render_widget(heirarchy, chunks[0]);
        f.render_widget(render, chunks[1]);
    })?;
    Ok(())
}
