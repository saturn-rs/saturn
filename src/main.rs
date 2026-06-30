use std::io;

use crate::{
    app::{App, Mode},
    widgets::prepare::Prepare,
};

mod app;
mod buffer;
mod events;
mod widgets;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new(Mode::Prepare(Prepare::new())).run(terminal))
}
