use chrono::prelude::Local;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct DateTimeComponent;

impl DateTimeComponent {
    pub fn new() -> DateTimeComponent {
        DateTimeComponent::default()
    }
}

impl Widget for &DateTimeComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let datetime_block = Block::new()
            .borders(Borders::BOTTOM)
            .border_set(border::THICK);

        let current_day: String = Local::now().format("%A").to_string();
        let current_date: String = Local::now().format("%d/%m/%Y").to_string();
        let current_time: String = Local::now().format("%H:%M").to_string();

        let mut lines: Vec<Line> = Vec::new();
        for _ in 1..(area.height - 3) / 2 {
            lines.push(Line::from(""))
        }

        lines.push(Line::from(current_time).bold().centered());
        lines.push(Line::from(current_day).blue().centered().bold());
        lines.push(Line::from(current_date).bold().centered());

        let datetime_text = Text::from(lines);

        Paragraph::new(datetime_text)
            .block(datetime_block)
            .render(area, buf);
    }
}
