use crate::app::App;
use crate::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

pub async fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    //setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //create app and run it
    let app = App::new(String::from("DGG Chat"), enhanced_graphics).await;
    let res = run_app(&mut terminal, app, tick_rate).await;

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(&app, f))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
                    // KeyCode::Left => ui.on_left(),
                    // KeyCode::Up => ui.on_up(),
                    // KeyCode::Right => ui.on_right(),
                    // KeyCode::Down => ui.on_down(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick().await;
            last_tick = Instant::now();
        }
        if app.do_close {
            return Ok(());
        }
    }
}
