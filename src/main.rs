#![allow(dead_code, unused_imports)]

mod card;
mod blackjack;

use crossterm::event::{self, KeyCode};
use hashish::HashSet;
use rand::{rng, Rng};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use crate::blackjack::Worth;
use crate::card::{Card, Deck, Suit};

fn main() -> std::io::Result<()> {
    let terminal = ratatui::init();

    let app = App {
        state: AppState::default(),
        player: {
            let count = 2;
            let mut cards = HashSet::with_capacity(count);

            for _ in 0..count {
                cards.insert(rng().random::<Card>());
            }

            Deck::new(cards)
        }
    };

    app.run(terminal)?;
    ratatui::restore();

    Ok(())
}

enum Alignment {
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom(u16, u16),
    Percent(f32, f32),
}

fn align_rect(parent: Rect, size: (u16, u16), alignment: Alignment) -> Rect {
    let (w, h) = (size.0, size.1);

    let x = match alignment {
        Alignment::Left => 0,
        Alignment::Center => (parent.width.saturating_sub(w)) / 2,
        Alignment::Right => parent.width.saturating_sub(w),
        Alignment::TopLeft | Alignment::Top => 0,
        Alignment::TopRight => parent.width.saturating_sub(w),
        Alignment::BottomLeft | Alignment::Bottom => 0,
        Alignment::BottomRight => parent.width.saturating_sub(w),
        Alignment::Custom(x, _) => x,
        Alignment::Percent(px, _) => ((parent.width as f32 - w as f32) * px) as u16,
    };

    let y = match alignment {
        Alignment::Top | Alignment::TopLeft | Alignment::TopRight => 0,
        Alignment::Left | Alignment::Right | Alignment::Center => (parent.height.saturating_sub(h)) / 2,
        Alignment::Bottom | Alignment::BottomLeft | Alignment::BottomRight => parent.height.saturating_sub(h),
        Alignment::Custom(_, y) => y,
        Alignment::Percent(_, py) => ((parent.height as f32 - h as f32) * py) as u16,
    };

    Rect::new(parent.x + x, parent.y + y, w, h)
}

struct App {
    state: AppState,
    player: Deck,
}

#[derive(Default, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Cancelled,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| {
                self.render(frame)
            })?;

            self.handle_events()?;
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let mount = Block::bordered()
            .title("| Blank Jack |")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Red));

        frame.render_widget(mount, frame.area());

        let worth = Line::raw(format!("Sum: {:?}", self.player.worth()));

        let worth_rect = align_rect(frame.area(), (7, 1), Alignment::Percent(0.125, 0.15));
        frame.render_widget(worth, worth_rect);

        for (index, card) in self.player.cards.iter().enumerate() {
            let x = 0.075 * ((index + 1) as f32);
            let card_rect = align_rect(frame.area(), (15, 9), Alignment::Percent(x, 0.5));

            render_card(frame, card_rect, *card);
        }

        self.help(frame);
    }

    fn help(&mut self, frame: &mut Frame) {
        let block = Block::bordered();
        let block_rect = align_rect(frame.area(), (45, 3), Alignment::Percent(0.015, 0.95));
        let message = Line::raw("Press <q> to quit and <Enter> to shuffle.");
        let message_rect = align_rect(frame.area(), (41, 1), Alignment::Percent(0.0225, 0.90));

        frame.render_widget(block, block_rect);
        frame.render_widget(message, message_rect);
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => self.state = AppState::Cancelled,
                KeyCode::Enter => {
                    let count = 2;
                    let mut cards = HashSet::with_capacity(count);

                    for _ in 0..count {
                        cards.insert(rng().random::<Card>());
                    }

                    self.player = Deck::new(cards);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn render_card(f: &mut Frame, rect: Rect, card: Card) {
    let width = rect.width as usize;
    let height = rect.height as usize;
    let style = Style::default().fg(suit_color(card.suit));
    let mut lines = Vec::new();

    for index in 0..height {
        if index == 0 {
            lines.push(
                Line::from(
                    Span::styled(
                        format!(" {}", card.value),
                        style,
                    )
                )
            );
        } else if index == height - 3 {
            lines.push(
                Line::from(
                    Span::styled(
                        format!("{}{}", " ".repeat(width - 4), card.value),
                        style,
                    )
                )
            );
        } else if index == height / 2 - 1{
            lines.push(
                Line::from(
                    Span::styled(
                        format!("{}{}", " ".repeat(width / 2 - 1), card.suit),
                        style,
                    )
                )
            );
        } else {
            lines.push(
                Line::from(Span::raw(""))
            );
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, rect);
}

fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Spades | Suit::Clubs => Color::White,
        Suit::Hearts | Suit::Diamonds => Color::Red,
    }
}