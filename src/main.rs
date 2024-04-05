mod app;
mod ui;
mod board;

use std::panic;

use app::App;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event};
use crossterm::event::Event::Key;
use anyhow::Result;
use ratatui::{Terminal, backend::{CrosstermBackend, Backend}};

fn main() -> Result<()>{
    startup()?;
    
    let app = App::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let app_exit = run_app(app, &mut terminal);

    shutdown()?;

    if let Err(err) = app_exit {
        println!("{err:?}");
    } 

    Ok(())
}

fn run_app<B: Backend>(mut app: App, terminal: &mut Terminal<B>) -> Result<()> {
    loop {
        terminal.draw(|f| {ui::render(f, &app)})?; 

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {

                    if app.game_over {
                        match key.code {
                            event::KeyCode::Char('q') => app.should_quit = true,
                            event::KeyCode::Char('r') => app.restart(),
                            _ => {},
                        }
                    } else {
                        match key.code {
                            event::KeyCode::Char('q') => app.should_quit = true,
                            event::KeyCode::Char('r') => app.restart(),
                            event::KeyCode::Up => app.move_selection(app::Direction::Up),
                            event::KeyCode::Down => app.move_selection(app::Direction::Down),
                            event::KeyCode::Left => app.move_selection(app::Direction::Left),
                            event::KeyCode::Right => app.move_selection(app::Direction::Right),
                            event::KeyCode::Char(' ') => app.play_on_selection(),
                            _ => {},
                        }
                    }
                }
            }
        }

        // application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}


fn startup() -> Result<()> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;

    let prev_panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |p_info| {
        shutdown().expect("terminal should be able to reset to original state");
        prev_panic_hook(p_info);
    }));

    Ok(())
}

// shutdown down: reset terminal back to original state
fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}