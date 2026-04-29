#![allow(dead_code, unused_imports)]

mod blackjack;
mod card;
mod holdem;
mod menu;
mod play;
mod poker;
mod show;

use {
    crate::{menu::Menu, play::Play},
    crossterm::event::{self, Event},
    ratatui::{
        layout::Rect,
        style::{Color, Style},
        widgets::{Block, BorderType, Borders},
        DefaultTerminal, Frame,
    },
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut game = Menu::new();
    run(terminal, &mut game)?;
    ratatui::restore();
    Ok(())
}

fn run(mut terminal: DefaultTerminal, game: &mut dyn Play) -> std::io::Result<()> {
    while game.active() {
        terminal.draw(|frame| render(frame, game))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                game.handle(key);
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, game: &mut dyn Play) {
    let area = frame.area();
    frame.render_widget(ratatui::widgets::Clear, area);
    let mount = Block::bordered()
        .title("| Cards |")
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(mount, area);
    game.render(frame, area);
}
