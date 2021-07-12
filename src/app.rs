use tui::widgets::{Widget, Block, Borders, List, ListItem, ListState};
use tui::style::{Style, Modifier};

pub struct Account {
    pub name: String,
    pub folders: Folders,
}

impl Account {
    pub fn new(name: &str, folders: Vec<String>) -> Account {
        Self {
            name: String::from(name),
            folders: Folders::new(folders),
        }
    }
}

pub struct AccountsList {
    pub accounts: Vec<Account>,
    pub state: ListState
}

impl AccountsList {
    pub fn new(accounts: Vec<Account>) -> AccountsList {
        let mut default_state = ListState::default();
        default_state.select(Some(0));
        Self {
            accounts,
            state: default_state 
        }
    }

    pub fn set_items(&mut self, new_accounts: Vec<Account>) {
        self.accounts = new_accounts;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                let length = self.accounts.len();
                let mut account = &mut self.accounts[i];
                let sub_i = match account.folders.state.selected() {
                    Some(si) => si,
                    None => 0
                };
                if i >= length - 1 {
                    if sub_i >= account.folders.items.len() - 1 {
                        account.folders.unselect();
                        0
                    } else {
                        account.folders.next();
                        i
                    }
                } else {
                    if sub_i >= account.folders.items.len() - 1 {
                        account.folders.unselect();
                        i + 1
                    } else {
                        account.folders.next();
                        i
                    }
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                let length = self.accounts.len();
                let mut account = &mut self.accounts[i];
                let sub_i = match account.folders.state.selected() {
                    Some(si) => si,
                    None => 0
                };
                if i == 0 {
                    if sub_i == 0 {
                        account.folders.unselect();
                        self.accounts.len() - 1
                    } else {
                        account.folders.previous();
                        i
                    }
                } else {
                    if sub_i == 0 {
                        account.folders.unselect();
                        i - 1
                    } else {
                        account.folders.previous();
                        i
                    }
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
                // Wrap back to end
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
    pub accounts: AccountsList,
}

impl App {
    pub fn new(accounts: Vec<Account>) -> App {

        Self {
            selection: Selection::Heirarchy,
            accounts: AccountsList::new(accounts),
        }
    }

    pub fn on_down(&mut self) {
        match self.selection {
            Selection::Heirarchy => {
                self.accounts.next()
            },
            _ => {}
        }
    }

    pub fn on_up(&mut self) {
        match self.selection {
            Selection::Heirarchy => {
                self.accounts.previous()
            },
            _ => {}
        }
    }
}
