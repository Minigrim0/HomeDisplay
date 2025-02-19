use chrono::prelude::Local;
use chrono::{FixedOffset, Utc};
use common::settings::TimezoneData;
use log::error;
use ratatui::text::ToLine;
use std::time::SystemTime;

use ratatui::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Debug)]
/// This component displays local time as well as optional offseted timezones
pub struct DateTimeComponent {
    pub timezones: Vec<TimezoneData>,  // Direction (E, W), offset (in hours) and Name
    pub currently_displayed_offset: u32,
    pub last_offset_change: SystemTime,
}

impl Default for DateTimeComponent {
    fn default() -> Self {
        Self {
            timezones: Vec::new(),
            currently_displayed_offset: 0,
            last_offset_change: SystemTime::now()
        }
    }
}

impl DateTimeComponent {
    pub fn new(timezones: Vec<TimezoneData>) -> Self {
        Self {
            timezones,
            currently_displayed_offset: 0,
            last_offset_change: SystemTime::now(),
        }
    }

    pub fn push_timezone(&mut self, timezone: TimezoneData) {
        self.timezones.push(timezone);
    }

    pub fn advance_timezone(&mut self) {
        self.currently_displayed_offset = (self.currently_displayed_offset + 1) % self.timezones.len() as u32;
        self.last_offset_change = SystemTime::now();
    }
}

impl DateTimeComponent {
    fn render_local(&self, frame: Rect, buf: &mut Buffer) {
        let today = Local::now();
        let current_day: String = today.format("%A").to_string();
        let current_date: String = today.format("%d/%m/%Y").to_string();
        let current_time: String = today.format("%H:%M").to_string();

        let mut datetime_text = vec![];

        let empty_lines = (frame.height as i32 - 3) / 2;
        for _ in 0..empty_lines {
            datetime_text.push(Line::from(""));
        }

        datetime_text.extend(vec![
            Line::from(current_day).bold().centered(),
            Line::from(current_date).blue().bold().centered(),
            Line::from(current_time).bold().centered()
        ]);
        let datetime_text = Text::from(datetime_text);

        let datetime_block = Block::new();

        Paragraph::new(datetime_text)
            .block(datetime_block)
            .alignment(Alignment::Center)
            .render(frame, buf);
    }

    /// Renders the currently displayed timezone. Different timezones are stored in the `timezones` field.
    fn render_current_timezone(&self, frame: Rect, buf: &mut Buffer) -> Result<(), String> {
        let (offseted_timezone, timezone_name) = if let Some(timezone) = self.timezones.get(self.currently_displayed_offset as usize) {
            match timezone.direction.to_uppercase().as_str() {
                "E" => {
                    (FixedOffset::east_opt((timezone.offset * 3600.0) as i32)
                        .ok_or(format!("Unable to build a valid offset with {} seconds !", timezone.offset))?, &timezone.name)
                }
                "W" => {
                    (FixedOffset::west_opt(timezone.offset as i32)
                        .ok_or(format!("Unable to build a valid offset with {} seconds !", timezone.offset))?, &timezone.name)
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

        let mut datetime_text = vec![];

        let empty_lines = (frame.height as i32 - 4) / 2;
        for _ in 0..empty_lines {
            datetime_text.push(Line::from(""));
        }

        datetime_text.extend(vec![
            timezone_name.to_line().bold().underlined().centered(),
            Line::from(current_day).bold().centered(),
            Line::from(current_date).blue().bold().centered(),
            Line::from(current_time).bold().centered()
        ]);
        let datetime_text = Text::from(datetime_text);

        Paragraph::new(datetime_text)
            .block(datetime_block)
            .alignment(Alignment::Center)
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
