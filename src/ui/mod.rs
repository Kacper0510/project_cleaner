use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

use crate::args::*;

mod app;
use app::App;

mod event;
use event::{Event, EventHandler};

mod handler;
use handler::handle_key_events;

mod render;

mod tui;
use tui::Tui;

pub fn run(args: Args) -> AppResult<()> {
    // Create an application.
    let mut app = App::new(args);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
