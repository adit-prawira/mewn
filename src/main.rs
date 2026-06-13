use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use mewn::cat::Cat;
use mewn::connections::store::ConnectionStore;
use mewn::dashboard::Dashboard;
use mewn::terminal::Terminal;

#[tokio::main]
async fn main() {   
    let mut terminal = Terminal::init();
    let mut cat = Cat::default();

    let connection_store = ConnectionStore::default();
    let shared_connections = connection_store.watch().await;
    
    let mut dashboard = Dashboard::default(); 
    dashboard.set_shared_connections(shared_connections);
    
    loop {
        cat.animate(&mut terminal);
        if event::poll(Duration::from_millis(50)).expect("poll failed") 
            && let Ok(Event::Key(_)) = event::read(){
                break; 
        }

        if cat.is_complete() {
            break;
        }
    }

    terminal.clear_screen().expect("failed to clear screen");

    loop {
        dashboard.render(&mut terminal); 

        if event::poll(Duration::from_millis(50)).expect("poll failed")
            && let Ok(Event::Key(key)) = event::read()
        {
            match key.code {
                KeyCode::Char('q') => return,
                KeyCode::Tab => dashboard.next_tab(),
                _ => {
                    dashboard.handle_keys(key.code);
                }
            }
        }
    }
}
