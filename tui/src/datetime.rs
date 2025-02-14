use chrono::prelude::Local;
use chrono::FixedOffset;

use ratatui::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};
use log::error;
use chrono::Utc;

#[derive(Debug, Default)]
/// This component displays local time as well as optional offseted timezones
pub struct DateTimeComponent {
    pub timezones: Vec<(String, u32)>,  // Direction (E, W) and offset (e.g. 5 * 60 * 60 => 5h)
    pub currently_displayed_offset: u32,
}

impl DateTimeComponent {
    pub fn new() -> DateTimeComponent {
        DateTimeComponent::default()
    }
}

impl DateTimeComponent {
    fn render_local(&self, frame: Rect, buf: &mut Buffer) {
        let today = Local::now();
        let current_day: String = today.format("%A").to_string();
        let current_date: String = today.format("%d/%m/%Y").to_string();
        let current_time: String = today.format("%H:%M").to_string();

        let datetime_text = Text::from(vec![
            Line::from(current_day).bold().centered(),
            Line::from(current_date).blue().bold().centered(),
            Line::from(current_time).bold().centered()
        ]);

        let datetime_block = Block::new()
            .borders(Borders::BOTTOM)
            .border_set(border::THICK);
        
        Paragraph::new(datetime_text)
            .block(datetime_block)
            .render(frame, buf);
    }

    fn render_current_timezone(&self, frame: Rect, buf: &mut Buffer) -> Result<(), String> {
        let offseted_timezone = if let Some(offset) = self.timezones.get(self.currently_displayed_offset as usize) {
            match offset.0.to_uppercase().as_str() {
                "E" => {
                    FixedOffset::east_opt(offset.1 as i32)
                        .ok_or(format!("Unable to build a valid offset with {} seconds !", offset.1))?
                }
                "W" => {
                    FixedOffset::west_opt(offset.1 as i32)
                        .ok_or(format!("Unable to build a valid offset with {} seconds !", offset.1))?
                }
                other => {
                    error!("Incorrect timezone offset: {other}");
                    Err(format!("Incorrect timezone offset `{other}`. Expected one of 'E', 'W'"))?
                }
            }
        } else {
            // No timezone to display
            return Ok(());
        };
        let datetime_block = Block::new()
            .borders(Borders::BOTTOM)
            .border_set(border::THICK);

        let today = Utc::now().with_timezone(&offseted_timezone);
        let current_day: String = today.format("%A").to_string();
        let current_date: String = today.format("%d/%m/%Y").to_string();
        let current_time: String = today.format("%H:%M").to_string();

        let mut lines: Vec<Line> = Vec::new();
        let datetime_text = Text::from(vec![
            Line::from(current_day).bold().centered(),
            Line::from(current_date).blue().bold().centered(),
            Line::from(current_time).bold().centered()
        ]);

        Paragraph::new(datetime_text)
            .block(datetime_block)
            .render(frame, buf);

        Ok(())
    }
}

impl Widget for &DateTimeComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let splitted = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 2); 2
        ])
            .split(area);


        self.render_local(splitted[0], buf);
        if let Err(e) = self.render_current_timezone(splitted[1], buf) {
            error!("Error while rendering offseted timezone: {e}");
        }
    }
}
