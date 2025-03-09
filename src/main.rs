use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    text::Text,
    Frame,
    widgets::Block,
    DefaultTerminal
};

fn main() -> std::io::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();

    result
}


fn run(mut terminal: DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(draw);
        handle_events();
        }
}

fn draw(frame: &mut Frame) {
    let block = Block::new().title("Title");
    frame.render_widget(block, frame.area());
}


fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}
