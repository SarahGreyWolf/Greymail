use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
    error::Error,
};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{self, EnableMouseCapture, DisableMouseCapture, Event as CEvent, KeyCode};
use crossterm::execute;
use tui::Terminal;
use tui::backend::{CrosstermBackend, Backend};
use tui::widgets::{Widget, Block, Borders, List, ListItem};
use tui::style::{Style, Modifier};
use tui::layout::{Layout, Constraint, Direction};

enum Event<I> {
    Input(I),
    Tick
}

#[derive(Debug)]
struct Cli {
    tick_rate: u64,
    enhanced_graphics: bool,
}

impl Cli {
    pub fn new(tick_rate: u64, eh_graphics: bool) -> Cli {
        Self {
            tick_rate,
            enhanced_graphics: eh_graphics
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::new(250, true);
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let mut backend = CrosstermBackend::new(stdout);
    backend.clear()?;
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(cli.tick_rate);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
    
    loop {
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
            let items = [ListItem::new("Test")];
    
            let heirarchy = List::new(items)
                .block(Block::default()
                    .borders(Borders::RIGHT)
                )
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">");
    
            let render = Block::default();
    
            // Create a bordered block
            let window = Block::default()
                .title("Greymail")
                .borders(Borders::ALL);
    
            f.render_widget(window, size);
            f.render_widget(heirarchy, chunks[0]);
            f.render_widget(render, chunks[1]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    break;
                },
                _ => {}
            },
            Event::Tick => {

            }
        }
    }
    quit(terminal.backend_mut())
}

fn quit<W: io::Write>(term: &mut CrosstermBackend<W>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(term, LeaveAlternateScreen, DisableMouseCapture)?;
    term.show_cursor()?;
    Ok(())
}
