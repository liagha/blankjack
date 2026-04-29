// src/main.rs
#![allow(dead_code, unused_imports)]

mod card;
mod blackjack;
mod show;

use {
    crossterm::event::{self, KeyCode, KeyEvent},
    ratatui::{
        layout::{Margin, Rect},
        style::{Color, Style},
        widgets::{Block, BorderType, Borders, Paragraph},
        prelude::*,
        DefaultTerminal,
    },
    crate::{
        show::{
            render_card_at,
            render_hand, render_sum, render_hidden_card, suit_color,
        },
        card::{Worth, Shoe, Hand, Card, Suit},
    },
};

struct App {
    phase: Phase,
    shoe: Shoe,
    player_hand: Hand,
    dealer_hand: Hand,
    money: i32,
    bet: i32,
    message: String,
}

#[derive(PartialEq)]
enum Phase {
    Betting,
    PlayerTurn,
    DealerTurn,
    Result,
    End,
}

fn main() -> std::io::Result<()> {
    let terminal = ratatui::init();
    let mut shoe = Shoe::new(4);
    shoe.shuffle();

    let app = App {
        phase: Phase::Betting,
        shoe,
        player_hand: Hand::new(),
        dealer_hand: Hand::new(),
        money: 1000,
        bet: 10,
        message: String::new(),
    };

    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        while self.phase != Phase::End {
            terminal.draw(|frame| self.render(frame))?;
            self.handle()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let mount = Block::bordered()
            .title("| Blank Jack |")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(mount, frame.area());

        let inner = frame.area().inner(Margin::new(1, 1));

        self.status(frame, inner);
        self.help(frame, inner);

        match self.phase {
            Phase::PlayerTurn | Phase::DealerTurn | Phase::Result => {
                self.game_screen(frame, inner);
            }
            _ => {}
        }
    }

    fn status(&self, frame: &mut Frame, inner: Rect) {
        let money = Line::raw(format!("Money: ${}", self.money));
        let bet = Line::raw(format!("Bet: ${}", self.bet));
        frame.render_widget(money, Rect::new(inner.x + 2, inner.y + 1, 14, 1));
        frame.render_widget(bet, Rect::new(inner.x + 2, inner.y + 3, 10, 1));

        if !self.message.is_empty() {
            let msg = Line::raw(&self.message)
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(msg, Rect::new(inner.x + inner.width / 2 - 15, inner.y + 1, 30, 1));
        }
    }

    fn help(&mut self, frame: &mut Frame, inner: Rect) {
        let lines = match self.phase {
            Phase::Betting => vec![
                Line::raw("[↑↓] Adjust Bet"),
                Line::raw("[Enter] Deal"),
                Line::raw("[Q] Quit"),
            ],
            Phase::PlayerTurn => vec![
                Line::raw("[H] Hit"),
                Line::raw("[D] Double"),
                Line::raw("[S] Stand"),
                Line::raw("[Q] Quit"),
            ],
            Phase::Result => vec![
                Line::raw("[Enter] Next Round"),
                Line::raw("[Q] Quit"),
            ],
            _ => vec![Line::raw("")],
        };

        let base_y = inner.y + inner.height - 5;
        for (i, line) in lines.iter().enumerate() {
            let rect = Rect::new(inner.x + 2, base_y + i as u16, inner.width - 4, 1);
            frame.render_widget(line.clone(), rect);
        }
    }

    fn game_screen(&mut self, frame: &mut Frame, inner: Rect) {
        let dealer_area = Rect::new(inner.x + 2, inner.y + 2, inner.width - 4, 12);
        let dealer_sum_area = Rect::new(dealer_area.x, dealer_area.y + dealer_area.height, dealer_area.width, 1);

        let player_area = Rect::new(inner.x + 2, inner.y + inner.height - 16, inner.width - 4, 12);
        let player_sum_area = Rect::new(player_area.x, player_area.y - 1, player_area.width, 1);

        let hide_dealer = self.phase == Phase::PlayerTurn;

        render_hand(frame, &self.dealer_hand, dealer_area, hide_dealer);
        if !hide_dealer {
            render_sum(frame, self.dealer_hand.worth(), dealer_sum_area);
        }

        render_hand(frame, &self.player_hand, player_area, false);
        render_sum(frame, self.player_hand.worth(), player_sum_area);
    }

    fn handle(&mut self) -> std::io::Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match self.phase {
                Phase::Betting => self.handle_betting(key),
                Phase::PlayerTurn => self.handle_player(key),
                Phase::Result => self.handle_result(key),
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_betting(&mut self, key: KeyEvent) {
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
                self.phase = Phase::PlayerTurn;
            }
            _ => {}
        }
    }

    fn deal(&mut self) {
        self.player_hand.cards.clear();
        self.dealer_hand.cards.clear();
        self.message.clear();

        if self.shoe.remaining() < 20 {
            self.shoe = Shoe::new(4);
            self.shoe.shuffle();
        }

        for _ in 0..2 {
            self.player_hand.add(self.shoe.draw().unwrap());
            self.dealer_hand.add(self.shoe.draw().unwrap());
        }
    }

    fn handle_player(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.phase = Phase::End;
            }
            KeyCode::Char('h') => {
                self.player_hand.add(self.shoe.draw().unwrap());
                if self.player_hand.worth() > 21 {
                    self.message = "Bust!".into();
                    self.resolve();
                    self.phase = Phase::Result;
                }
            }
            KeyCode::Char('s') => {
                self.dealer_play();
                self.phase = Phase::Result;
            }
            KeyCode::Char('d') => {
                if self.player_hand.cards.len() == 2 && self.money >= self.bet * 2 {
                    self.bet *= 2;
                    self.player_hand.add(self.shoe.draw().unwrap());
                    if self.player_hand.worth() > 21 {
                        self.message = "Bust!".into();
                        self.resolve();
                        self.phase = Phase::Result;
                    } else {
                        self.dealer_play();
                        self.phase = Phase::Result;
                    }
                }
            }
            _ => {}
        }
    }

    fn dealer_play(&mut self) {
        while self.dealer_hand.worth() < 17 {
            self.dealer_hand.add(self.shoe.draw().unwrap());
        }
        self.resolve();
    }

    fn resolve(&mut self) {
        let player = self.player_hand.worth();
        let dealer = self.dealer_hand.worth();

        let player_blackjack = player == 21 && self.player_hand.cards.len() == 2;
        let dealer_blackjack = dealer == 21 && self.dealer_hand.cards.len() == 2;

        if player > 21 {
            self.message = "Bust! You lose.".into();
            self.money -= self.bet;
        } else if dealer > 21 {
            self.message = "Dealer busts! You win.".into();
            self.money += self.bet;
        } else if player_blackjack && !dealer_blackjack {
            self.message = "Blackjack!".into();
            self.money += (self.bet as f32 * 1.5) as i32;
        } else if dealer_blackjack && !player_blackjack {
            self.message = "Dealer blackjack. You lose.".into();
            self.money -= self.bet;
        } else if player > dealer {
            self.message = "You win.".into();
            self.money += self.bet;
        } else if dealer > player {
            self.message = "You lose.".into();
            self.money -= self.bet;
        } else {
            self.message = "Push.".into();
        }

        if self.money <= 0 {
            self.message.push_str(" No money left!");
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
                    self.phase = Phase::Betting;
                }
            }
            _ => {}
        }
    }
}