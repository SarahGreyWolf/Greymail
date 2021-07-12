use crate::State;
use crate::app::App;
use tui::widgets::{Widget, Block, Borders, List, ListItem};
use tui::style::{Style, Modifier};
use tui::layout::{Layout, Constraint, Direction};
use tui::backend::Backend;
use tui::terminal::Frame;


pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

    let items: Vec<ListItem> = app
        .folders.items.iter()
        .map(|i| ListItem::new(i.as_ref()))
        .collect();
    let heirarchy = List::new(items)
        .block(Block::default()
            .title("m.sarahgreywolf@outlook.com")
            .borders(Borders::RIGHT)
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    let render = Block::default();
    
    // Create a bordered block
    let window = Block::default()
        .title("Greymail")
        .borders(Borders::ALL);
   
    f.render_widget(window, size);
    f.render_stateful_widget(heirarchy, chunks[0], &mut app.folders.state);
    f.render_widget(render, chunks[1]);
}
