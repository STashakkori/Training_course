use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget},
    Frame,
};

use crate::app::App;
use ratatui::prelude::Line;
use ratatui::prelude::Rect;
use ratatui::prelude::Buffer;
use ratatui::prelude::Span;

#[derive(Debug, Clone)]
pub enum State {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
           text: Color::Black,
           background: Color::Yellow,
           highlight: Color::Reset,
           shadow: Color::Reset,
        }
    }
}

#[derive(Debug, Clone)]
struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: State,
}

impl<'a> Button<'a> {
    pub fn new(label: &str) -> Button {
        Button {
            label: Line::from(label),
            theme: Theme::default(), // Ensure Theme has a default implementation
            state: State::Normal,
        }
    }

    pub fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text_width = self.label.width() as u16 + 4; // Example padding
        let center_x = area.x + (area.width.saturating_sub(text_width + 2)) / 2;

        let adjusted_area = Rect {
            x: center_x,
            y: area.y,
            width: std::cmp::min(text_width, area.width),
            height: area.height,
        };

        let style = match self.state {
            State::Normal => Style::default().fg(self.theme.text).bg(self.theme.background),
            _ => Style::default(), // Handle other states if necessary
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(style);

        block.render(adjusted_area, buf);

        let spans = self.label.spans.clone();
        let text = spans.iter().map(|span| span.content.clone()).collect::<Vec<_>>().join(" ");
        let span = Span::raw(text);
        let paragraph = Paragraph::new(span)
            .style(style)
            .alignment(Alignment::Center);
        
        let paragraph_area = Rect {
            x: adjusted_area.x + 1, // Add padding for the border
            y: adjusted_area.y + 1,
            width: adjusted_area.width.saturating_sub(2), // Reduce width for borders
            height: adjusted_area.height.saturating_sub(2),
        };

        paragraph.render(paragraph_area, buf);
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(6), // For the title
            Constraint::Length(2),   // For the modules list
            Constraint::Min(8), // For the progress bar or progress text
            Constraint::Length(3), // For the Resume button
            Constraint::Length(3), // For the Resume button
        ])
        .split(size);

    let welcome_message = Paragraph::new(format!(
        "\nIn this course you'll learn the basics of cybersecurity.\n\nPress `Esc`, `Ctrl-C` or `q` to stop running. Time: ~1hr"
    ))
    .block(
        Block::default()
            .title("Course 1: Introduction to Cybersecurity")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightGreen))
            .borders(Borders::ALL)
            //.border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    .alignment(Alignment::Center);
    frame.render_widget(welcome_message, chunks[0]);

    let title = Paragraph::new("\nModules in this course:")
        .style(Style::default().fg(Color::LightCyan).bg(Color::Black))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(title, chunks[1]);
/*
    let items: Vec<ListItem> = app.modules.iter().enumerate().map(|(index, module)| {
    //let content = format!(" {}. {}\n\n", index, module); // Number modules
    let content = format!("{}\n\n", module); // Don't number
    ListItem::new(content)
        .style(Style::default().fg(Color::LightCyan))
    }).collect();
*/
    let mut sorted_modules = app.modules.clone();
    sorted_modules.sort(); // Sorts in ascending order by default

    let items: Vec<ListItem> = sorted_modules.iter().map(|module| {
        let content = format!("{}\n\n", module); // Formatted content for each module
        ListItem::new(content)
        .style(Style::default().fg(Color::LightCyan))
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.current_module_index));

    let modules_list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::LightCyan).bg(Color::Black))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::DarkGray));
    frame.render_widget(modules_list, chunks[2]);

    // Progress indicator
    let progress_text = format!("Current Progress: Module {} of {}", app.current_module_index, app.modules.len());
    let progress = Paragraph::new(progress_text)
        .style(Style::default().fg(Color::LightGreen).bg(Color::Black))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(progress, chunks[3]);

    let resume_button = Button::new("Hit ENTER to Begin")
        .theme(Theme::default()) // Define the theme according to your preference
        .state(State::Normal); // Set the initial state of the button
    frame.render_widget(resume_button, chunks[4]);
}
