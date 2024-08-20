use app::{App, AppResult};
use event::{Event, EventHandler};
use handler::{handle_key_events, handle_mouse_events};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tui::Tui;

use crate::args::Args;

mod app;
mod event;
mod handler;
mod model;
mod popup;
mod render;
mod tui;

pub fn run(args: Args) -> AppResult<()> {
    // Create an application.
    let mut app = App::new(args);
    app.run();

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
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(..) => {},
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
