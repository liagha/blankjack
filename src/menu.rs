use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Rect},
    text::Line,
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{blackjack::Blackjack, holdem::Holdem, play::Play};

pub struct Menu {
    pub game: Option<Box<dyn Play>>,
    pub index: usize,
    pub run: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            game: None,
            index: 0,
            run: true,
        }
    }
}

impl Play for Menu {
    fn handle(&mut self, key: KeyEvent) {
        if let Some(game) = &mut self.game {
            game.handle(key);
            if !game.active() {
                self.game = None;
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.run = false,
            KeyCode::Up | KeyCode::Char('w') => self.index = 0,
            KeyCode::Down | KeyCode::Char('s') => self.index = 1,
            KeyCode::Enter => {
                if self.index == 0 {
                    self.game = Some(Box::new(Blackjack::new()));
                } else {
                    self.game = Some(Box::new(Holdem::new()));
                }
            }
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        if let Some(game) = &mut self.game {
            game.render(frame, area);
            return;
        }

        let lines = vec![
            Line::raw(if self.index == 0 { "> Blackjack" } else { "  Blackjack" }),
            Line::raw(if self.index == 1 { "> Holdem" } else { "  Holdem" }),
        ];

        let item = Paragraph::new(lines).alignment(Alignment::Center);
        let spot = Rect::new(area.x, area.y + area.height / 2, area.width, 2);

        frame.render_widget(Clear, area);
        frame.render_widget(item, spot);
    }

    fn active(&self) -> bool {
        self.run
    }
}
