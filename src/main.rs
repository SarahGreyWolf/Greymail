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
use app::{Account, AccountsList};

mod ui;
mod app;

pub enum State {
    List,
    Render,
}

enum Event<I> {
    Input(I),
    State(State),
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

    let mut app: app::App = app::App::new(
        vec![
            Account::new("m.sarahgreywolf@outlook.com", vec![String::from("Inbox"), String::from("Outbox")]),
            Account::new("master0r0@me.com", vec![String::from("Inbox"), String::from("Outbox")]),
        ]
    );

    
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    break;
                },
                KeyCode::Down => {
                    app.on_down();
                },
                KeyCode::Up => {
                    app.on_up();
                },
                _ => {}
            },
            Event::Tick => {

            },
            _ => {}
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
