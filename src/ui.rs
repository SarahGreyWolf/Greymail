use crate::State;
use tui::widgets::{Widget, Block, Borders, List, ListItem};
use tui::style::{Style, Modifier};
use tui::layout::{Layout, Constraint, Direction};
use tui::backend::Backend;
use tui::terminal::Frame;


pub fn draw<B: Backend>(f: &mut Frame<B>) {
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
}
