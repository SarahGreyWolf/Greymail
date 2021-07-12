use tui::widgets::{Widget, Block, Borders, List, ListItem, ListState};
use tui::style::{Style, Modifier};

pub struct Folders {
    pub items: Vec<String>,
    pub state: ListState
}

impl Folders {
    pub fn new(items: Vec<String>) -> Folders {
        let mut default_state = ListState::default();
        default_state.select(Some(0));
        Self {
            items,
            state: default_state 
        }
    }

    pub fn set_items(&mut self, new_items: Vec<String>) {
        self.items = new_items;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                // Wrap back to 0
                if i >= self.items.len() -1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                // Wrap back to 0
                if i == 0{
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub enum Selection {
    Heirarchy,
    Render
}

pub struct App {
    selection: Selection,
    pub folders: Folders,
}

impl App {
    pub fn new(folders: Vec<String>) -> App {
        Self {
            selection: Selection::Heirarchy,
            folders: Folders::new(folders),
        }
    }

    pub fn on_down(&mut self) {
        match self.selection {
            Selection::Heirarchy => {
                self.folders.next()
            },
            _ => {}
        }
    }

    pub fn on_up(&mut self) {
        match self.selection {
            Selection::Heirarchy => {
                self.folders.previous()
            },
            _ => {}
        }
    }
}
