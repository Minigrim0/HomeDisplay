use std::io;
use std::default::Default;

mod app;
mod currency;
mod datetime;
mod layout;
mod transports;
mod tui;
mod utilities;
mod weather;

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
