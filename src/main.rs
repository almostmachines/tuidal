use crossterm::event::{KeyCode, KeyEvent};
use rxrust::Observable;
use std::io;
use tuidal::{
    input::input_stream_factory::input_stream_factory, render::renderer::Renderer, state::State,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut renderer = Renderer::new(&mut terminal);
    let input_stream = input_stream_factory();

    renderer.render(State::default());
    input_stream
        .take_while(|key_evt: &KeyEvent| {
            key_evt.code != KeyCode::Char('q') && key_evt.code != KeyCode::Char('Q')
        })
        .subscribe(|_| renderer.render(State::default()));
    ratatui::restore();
    Ok(())
}
