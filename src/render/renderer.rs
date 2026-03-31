use crate::state::State;
use ratatui::{
    Terminal,
    prelude::CrosstermBackend,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};
use std::io::Stdout;

pub struct Renderer<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Renderer<'a> {
    pub fn new(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self { terminal }
    }

    pub fn render(&mut self, state: State) {
        let _ = self
            .terminal
            .draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();
                let title = Line::from(state.title.bold());

                let block = Block::bordered()
                    .title(title.centered())
                    .border_set(border::THICK);

                block.render(area, buf);
            })
            .inspect_err(|e| eprintln!("Rendering error: {e}"));
    }
}
