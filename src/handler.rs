use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use std::{fs, env};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            //app.increment_counter();
        }
        KeyCode::Left => {
            //app.decrement_counter();
        }
        KeyCode::Enter => {
            let r = "course.lrn";
            if !app.file_exists(r) {
                app.create_file_sorted(r);
            }
            if App::complete(app, r) {
                //app.crt();
                //app.quit();
            } else {
                //app.run_module(r)?;
                app.run_single("1_lecture")?;
                app.quit();
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn clean_file(file_path: String) {
     if fs::metadata(&file_path).is_ok() {
        match fs::remove_file(file_path) {
            Ok(_) => {},
            Err(e) => eprintln!("Error deleting course.lrn: {}", e),
        }
    }
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
        let click_x = mouse_event.column;
        let click_y = mouse_event.row;

        if app.is_click_on_resume_button(click_x, click_y) {
            app.resume_next_module();
        }
    }
    Ok(())
}
