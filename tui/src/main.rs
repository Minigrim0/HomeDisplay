use std::io;
use std::default::Default;

mod tui;
mod wrappers;
mod app;
mod utilities;
mod widget;

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}