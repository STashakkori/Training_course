use novum::app::{App, AppResult};
use novum::event::{Event, EventHandler};
use novum::handler::handle_key_events;
use novum::tui::Tui;
use std::io;
use std::io::Write;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode},
    ExecutableCommand,
    Result,
    terminal::{Clear, ClearType},
};
use crossterm::style::ResetColor;
use std::io::stdout;
//execute!(stdout(), ResetColor);
fn main() -> AppResult<()> {
    disable_raw_mode()?;
    //std::io::stdout().execute(ResetColor)?;
    clear_screen()?;
    let course_name = "IntroToCybersecurity"; // Set the course name here
    let mut app = App {
        course_name: course_name.to_string(),
        ..App::default()
    };

    std::io::stdout().flush().unwrap();
    app.load_modules();
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    //enable_raw_mode()?;
    let r = "course.lrn";

    // Manu loop
    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
              handle_key_events(key_event, &mut app)?
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
/*        
        if !app.file_exists(r) {
          app.create_file(r); 
        }
        if App::complete(&app, r) {
          app.crt();
        }
        else {
          app.run_module(r);
        }
        //if app.current_module_index == 0 {
        //app.next_module();
        //}
*/
    }
    // Exit the user interface.
    tui.exit()?;
    //disable_raw_mode()?;
    //enable_raw_mode()?;
    Ok(())
}

fn clear_screen() -> crossterm::Result<()> {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;
    //stdout.execute(Clear(ClearType::Purge))?;
    Ok(())
}
