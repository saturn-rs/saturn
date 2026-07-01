use std::io;

use crate::{
    app::{App, Mode},
    buffer::Buffer,
    widgets::{editor::Editor, prepare::Prepare},
};

mod app;
mod buffer;
mod events;
mod widgets;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        App::new(Mode::Edit(Editor::new(Buffer::new(
            "a\nb\nc".into(),
            "hello.gk".into(),
        ))))
        .run(terminal)
    })
}
