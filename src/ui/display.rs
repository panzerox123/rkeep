use std::io;
use termion::raw::IntoRawMode;
use tui::backend::Backend;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Frame;
use tui::Terminal;

pub fn init_term() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    display_terminal_ui(&mut terminal);
    Ok(())
}

fn display_terminal_ui<B>(term: &mut Terminal<B>)
where
    B: Backend,
{
    term.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(20),
                    Constraint::Percentage(70),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
        let block = Block::default().title("Block 3").borders(Borders::ALL);
        f.render_widget(block, chunks[2]);
    });
}
