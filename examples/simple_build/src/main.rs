use miette::IntoDiagnostic;
use saturn::{
    app::{App, Mode},
    buffer::Buffer,
    config::read_config,
    widgets::editor::Editor,
};

fn main() -> miette::Result<()> {
    let config = read_config().into_diagnostic()?;
    ratatui::run(|terminal| {
        App::new(
            Mode::Edit(Editor::new(
                Buffer::new("a\nb\nc".into(), "hello.gk".into()),
                "Ready for edit",
                &config.theme.edit,
            )),
            &config,
        )
        .run(terminal)
    })
    .into_diagnostic()
}
