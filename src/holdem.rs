use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{
    card::{Hand, Shoe},
    play::Play,
    poker::evaluate,
    show::render_hand,
};

#[derive(PartialEq)]
pub enum Phase {
    Ante,
    Preflop,
    Flop,
    Turn,
    River,
    Result,
    End,
}

pub struct Holdem {
    pub phase: Phase,
    pub shoe: Shoe,
    pub player: Hand,
    pub dealer: Hand,
    pub board: Hand,
    pub money: i32,
    pub bet: i32,
    pub pot: i32,
    pub message: String,
}

impl Holdem {
    pub fn new() -> Self {
        let mut shoe = Shoe::new(1);
        shoe.shuffle();
        Self {
            phase: Phase::Ante,
            shoe,
            player: Hand::new(),
            dealer: Hand::new(),
            board: Hand::new(),
            money: 1000,
            bet: 10,
            pot: 0,
            message: String::new(),
        }
    }

    fn deal(&mut self) {
        self.player.cards.clear();
        self.dealer.cards.clear();
        self.board.cards.clear();
        self.message.clear();
        self.pot = self.bet * 2;
        self.money -= self.bet;

        if self.shoe.remaining() < 15 {
            self.shoe = Shoe::new(1);
            self.shoe.shuffle();
        }

        for _ in 0..2 {
            self.player.add(self.shoe.draw().unwrap());
            self.dealer.add(self.shoe.draw().unwrap());
        }
    }

    fn resolve(&mut self) {
        let player = evaluate(&self.player, &self.board);
        let dealer = evaluate(&self.dealer, &self.board);

        if player > dealer {
            self.message = "Win".into();
            self.money += self.pot;
        } else if dealer > player {
            self.message = "Lose".into();
        } else {
            self.message = "Push".into();
            self.money += self.pot / 2;
        }
        self.pot = 0;

        if self.money <= 0 {
            self.message.push_str(" Bankrupt");
        }
    }

    fn handle_ante(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.phase = Phase::End,
            KeyCode::Up | KeyCode::Char('w') => {
                self.bet = (self.bet + 10).min(self.money);
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.bet = (self.bet - 10).max(10);
            }
            KeyCode::Enter => {
                if self.bet > self.money {
                    return;
                }
                self.deal();
                self.phase = Phase::Preflop;
            }
            _ => {}
        }
    }

    fn handle_action(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.phase = Phase::End,
            KeyCode::Char('f') => {
                self.message = "Fold".into();
                self.pot = 0;
                self.phase = Phase::Result;
            }
            KeyCode::Char('c') => {
                self.money -= self.bet;
                self.pot += self.bet * 2;
                self.advance();
            }
            _ => {}
        }
    }

    fn handle_result(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.phase = Phase::End,
            KeyCode::Enter => {
                if self.money <= 0 {
                    self.phase = Phase::End;
                } else {
                    self.bet = 10.min(self.money);
                    self.message.clear();
                    self.phase = Phase::Ante;
                }
            }
            _ => {}
        }
    }

    fn advance(&mut self) {
        match self.phase {
            Phase::Preflop => {
                for _ in 0..3 {
                    self.board.add(self.shoe.draw().unwrap());
                }
                self.phase = Phase::Flop;
            }
            Phase::Flop => {
                self.board.add(self.shoe.draw().unwrap());
                self.phase = Phase::Turn;
            }
            Phase::Turn => {
                self.board.add(self.shoe.draw().unwrap());
                self.phase = Phase::River;
            }
            Phase::River => {
                self.resolve();
                self.phase = Phase::Result;
            }
            _ => {}
        }
    }

    fn status(&self, frame: &mut Frame, area: Rect) {
        let money = Line::raw(format!("Money: ${}", self.money));
        let bet = Line::raw(format!("Bet: ${}", self.bet));
        let pot = Line::raw(format!("Pot: ${}", self.pot));
        let width = area.width.saturating_sub(2);
        frame.render_widget(Clear, area);
        frame.render_widget(money, Rect::new(area.x + 2, area.y, width, 1));
        frame.render_widget(bet, Rect::new(area.x + 2, area.y + 1, width, 1));
        frame.render_widget(pot, Rect::new(area.x + 2, area.y + 2, width, 1));
    }

    fn help(&self, frame: &mut Frame, area: Rect) {
        let lines = match self.phase {
            Phase::Ante => vec![
                Line::raw("[↑↓] Bet"),
                Line::raw("[Enter] Deal"),
                Line::raw("[Q] Quit"),
            ],
            Phase::Preflop | Phase::Flop | Phase::Turn | Phase::River => vec![
                Line::raw("[C] Call"),
                Line::raw("[F] Fold"),
                Line::raw("[Q] Quit"),
            ],
            Phase::Result => vec![
                Line::raw("[Enter] Next"),
                Line::raw("[Q] Quit"),
            ],
            _ => vec![Line::raw("")],
        };

        let y = area.y + area.height.saturating_sub(lines.len() as u16);
        let width = area.width.saturating_sub(4);
        frame.render_widget(Clear, area);
        for (i, line) in lines.iter().enumerate() {
            let item = Rect::new(area.x + 2, y + i as u16, width, 1);
            frame.render_widget(line.clone(), item);
        }
    }

    fn pop(&self, frame: &mut Frame, area: Rect) {
        if self.message.is_empty() {
            return;
        }
        let width = self.message.len() as u16 + 6;
        let x = area.x + area.width.saturating_sub(width) / 2;
        let y = area.y + area.height / 2;
        let spot = Rect::new(x, y, width, 1);

        let text = Line::raw(&self.message).style(Style::default().fg(Color::Black).bg(Color::White));
        let item = Paragraph::new(text).alignment(Alignment::Center);

        frame.render_widget(Clear, spot);
        frame.render_widget(item, spot);
    }

    fn scene(&mut self, frame: &mut Frame, area: Rect) {
        let chunk = area.height / 3;

        let dealer_rect = Rect::new(area.x, area.y, area.width, chunk);
        let board_rect = Rect::new(area.x, area.y + chunk, area.width, chunk);
        let player_rect = Rect::new(area.x, area.y + chunk * 2, area.width, chunk);

        let hide = self.phase != Phase::Result;

        render_hand(frame, &self.dealer, dealer_rect, hide);
        render_hand(frame, &self.board, board_rect, false);
        render_hand(frame, &self.player, player_rect, false);
    }
}

impl Play for Holdem {
    fn handle(&mut self, key: KeyEvent) {
        match self.phase {
            Phase::Ante => self.handle_ante(key),
            Phase::Preflop | Phase::Flop | Phase::Turn | Phase::River => self.handle_action(key),
            Phase::Result => self.handle_result(key),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let inner = area.inner(Margin::new(1, 1));

        let top = Rect::new(inner.x, inner.y, inner.width, 4);
        let bottom = Rect::new(inner.x, inner.y + inner.height.saturating_sub(4), inner.width, 4);
        let middle = Rect::new(inner.x, inner.y + 4, inner.width, inner.height.saturating_sub(8));

        self.status(frame, top);
        self.help(frame, bottom);

        match self.phase {
            Phase::Preflop | Phase::Flop | Phase::Turn | Phase::River | Phase::Result => {
                self.scene(frame, middle);
            }
            _ => {}
        }

        self.pop(frame, inner);
    }

    fn active(&self) -> bool {
        self.phase != Phase::End
    }
}
