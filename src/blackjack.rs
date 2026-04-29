use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Clear, Paragraph},
    Frame,
};
use crate::{
    card::{Hand, Shoe, Worth},
    play::Play,
    show::{render_hand, render_sum},
};

#[derive(PartialEq)]
pub enum Phase {
    Bet,
    Player,
    Dealer,
    Result,
    End,
}

pub struct Blackjack {
    pub phase: Phase,
    pub shoe: Shoe,
    pub player: Hand,
    pub dealer: Hand,
    pub money: i32,
    pub bet: i32,
    pub message: String,
}

impl Blackjack {
    pub fn new() -> Self {
        let mut shoe = Shoe::new(4);
        shoe.shuffle();
        Self {
            phase: Phase::Bet,
            shoe,
            player: Hand::new(),
            dealer: Hand::new(),
            money: 1000,
            bet: 10,
            message: String::new(),
        }
    }

    fn deal(&mut self) {
        self.player.cards.clear();
        self.dealer.cards.clear();
        self.message.clear();

        if self.shoe.remaining() < 20 {
            self.shoe = Shoe::new(4);
            self.shoe.shuffle();
        }

        for _ in 0..2 {
            self.player.add(self.shoe.draw().unwrap());
            self.dealer.add(self.shoe.draw().unwrap());
        }
    }

    fn play_dealer(&mut self) {
        while self.dealer.worth() < 17 {
            self.dealer.add(self.shoe.draw().unwrap());
        }
        self.resolve();
    }

    fn resolve(&mut self) {
        let player = self.player.worth();
        let dealer = self.dealer.worth();

        let player_win = player == 21 && self.player.cards.len() == 2;
        let dealer_win = dealer == 21 && self.dealer.cards.len() == 2;

        if player > 21 {
            self.message = "Bust".into();
            self.money -= self.bet;
        } else if dealer > 21 {
            self.message = "Win".into();
            self.money += self.bet;
        } else if player_win && !dealer_win {
            self.message = "Blackjack".into();
            self.money += (self.bet as f32 * 1.5) as i32;
        } else if dealer_win && !player_win {
            self.message = "Lose".into();
            self.money -= self.bet;
        } else if player > dealer {
            self.message = "Win".into();
            self.money += self.bet;
        } else if dealer > player {
            self.message = "Lose".into();
            self.money -= self.bet;
        } else {
            self.message = "Push".into();
        }

        if self.money <= 0 {
            self.message.push_str(" Bankrupt");
        }
    }

    fn handle_bet(&mut self, key: KeyEvent) {
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
                self.phase = Phase::Player;
            }
            _ => {}
        }
    }

    fn handle_player(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.phase = Phase::End;
            }
            KeyCode::Char('h') => {
                self.player.add(self.shoe.draw().unwrap());
                if self.player.worth() > 21 {
                    self.message = "Bust".into();
                    self.resolve();
                    self.phase = Phase::Result;
                }
            }
            KeyCode::Char('s') => {
                self.play_dealer();
                self.phase = Phase::Result;
            }
            KeyCode::Char('d') => {
                if self.player.cards.len() == 2 && self.money >= self.bet * 2 {
                    self.bet *= 2;
                    self.player.add(self.shoe.draw().unwrap());
                    if self.player.worth() > 21 {
                        self.message = "Bust".into();
                        self.resolve();
                        self.phase = Phase::Result;
                    } else {
                        self.play_dealer();
                        self.phase = Phase::Result;
                    }
                }
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
                    self.phase = Phase::Bet;
                }
            }
            _ => {}
        }
    }

    fn status(&self, frame: &mut Frame, area: Rect) {
        let money = Line::raw(format!("Money: ${}", self.money));
        let bet = Line::raw(format!("Bet: ${}", self.bet));
        let width = area.width.saturating_sub(2);
        frame.render_widget(Clear, area);
        frame.render_widget(money, Rect::new(area.x + 2, area.y, width, 1));
        frame.render_widget(bet, Rect::new(area.x + 2, area.y + 1, width, 1));
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

    fn help(&self, frame: &mut Frame, area: Rect) {
        let lines = match self.phase {
            Phase::Bet => vec![
                Line::raw("[↑↓] Bet"),
                Line::raw("[Enter] Deal"),
                Line::raw("[Q] Quit"),
            ],
            Phase::Player => vec![
                Line::raw("[H] Hit"),
                Line::raw("[D] Double"),
                Line::raw("[S] Stand"),
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

    fn scene(&mut self, frame: &mut Frame, area: Rect) {
        let chunk = area.height / 2;

        let dealer_area = Rect::new(area.x, area.y, area.width, chunk);
        let dealer_sum = Rect::new(dealer_area.x + 2, dealer_area.y + dealer_area.height.saturating_sub(1), dealer_area.width.saturating_sub(2), 1);

        let player_area = Rect::new(area.x, area.y + chunk, area.width, chunk);
        let player_sum = Rect::new(player_area.x + 2, player_area.y, player_area.width.saturating_sub(2), 1);

        let hide = self.phase == Phase::Player;

        render_hand(frame, &self.dealer, dealer_area, hide);
        if !hide {
            render_sum(frame, "Dealer", self.dealer.worth(), dealer_sum);
        }

        render_hand(frame, &self.player, player_area, false);
        render_sum(frame, "Player", self.player.worth(), player_sum);
    }
}

impl Play for Blackjack {
    fn handle(&mut self, key: KeyEvent) {
        match self.phase {
            Phase::Bet => self.handle_bet(key),
            Phase::Player => self.handle_player(key),
            Phase::Result => self.handle_result(key),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let inner = area.inner(Margin::new(1, 1));

        let top = Rect::new(inner.x, inner.y, inner.width, 3);
        let bottom = Rect::new(inner.x, inner.y + inner.height.saturating_sub(4), inner.width, 4);
        let middle = Rect::new(inner.x, inner.y + 3, inner.width, inner.height.saturating_sub(7));

        self.status(frame, top);
        self.help(frame, bottom);

        match self.phase {
            Phase::Player | Phase::Dealer | Phase::Result => {
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
