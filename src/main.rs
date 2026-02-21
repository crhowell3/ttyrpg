use clap::Parser;
use std::panic;
use ttyrpg::app::{App, AppResult};
use ttyrpg::event::{Event, EventHandler};
use ttyrpg::handler::handle_key_events;
use ttyrpg::ui::tui::Tui;

#[derive(Debug, Parser)]
#[clap(name = "ttyrpg")]
#[command(
    author = "Cameron Howell <me@crhowell.com>",
    version,
    about = "A rogue-like teletype terminal game.",
    help_template = "{name} {version}
author: {author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
struct Args {
    /// Toggle sound effects
    #[arg(long)]
    enable_audio: bool,
}

fn main() -> AppResult<()> {
    let _args = Args::parse();

    let mut app = App::default();

    let terminal = ratatui::try_init()?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        let _ = ratatui::crossterm::execute!(
            std::io::stdout(),
            ratatui::crossterm::event::DisableMouseCapture
        );
        default_panic(info);
    }));

    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    ratatui::try_restore()?;
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::DisableMouseCapture
    )?;

    Ok(())
}
