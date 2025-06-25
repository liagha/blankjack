#![allow(dead_code, unused_imports)]

mod card;
mod blackjack;
mod show;

use {
    rand::{rng, Rng},
    hashish::HashSet,
    crossterm::{
        event,
        event::KeyCode,
    },
    ratatui::{
        style::{Color, Style},
        widgets::{
            Block, BorderType,
            Borders, Paragraph,
        },
        prelude::*,
        
        DefaultTerminal,
    },
    
    crate::{
        show::{
            Alignment, Show,
            Formation,
        },
        blackjack::Worth,
        card::{
            Deck, Card, Suit,
        }
    }
};

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

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| {
                self.render(frame);
                    
                self.player.render(frame, Formation::default());
            })?;

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

        let worth = Line::raw(format!("Sum: {:?}", self.player.worth()));
        let worth_form = Formation::new((7, 1), Alignment::Percent(0.125, 0.15));
        let worth_rect = worth_form.rect(frame.area());
        frame.render_widget(worth, worth_rect);


        self.help(frame);
    }

    fn help(&mut self, frame: &mut Frame) {
        let hit = Line::raw("[A] Hit");
        let hit_form = Formation::new((7, 1), Alignment::Percent(0.025, 0.95));
        let hit_rect = hit_form.rect(frame.area());

        let double = Line::raw("[D] Double");
        let double_form = Formation::new((10, 1), Alignment::Percent(0.07, 0.95));
        let double_rect = double_form.rect(frame.area());

        let split = Line::raw("[S] Split");
        let split_form = Formation::new((9, 1), Alignment::Percent(0.13, 0.95));
        let split_rect = split_form.rect(frame.area());

        let stand = Line::raw("[F] Stand");
        let stand_form = Formation::new((9, 1), Alignment::Percent(0.1875, 0.95));
        let stand_rect = stand_form.rect(frame.area());

        let quit = Line::raw("[Q] Quit");
        let quit_form = Formation::new((8, 1), Alignment::Percent(0.24, 0.95));
        let quit_rect = quit_form.rect(frame.area());

        let shuffle = Line::raw("[Enter] Shuffle(Debug)");
        let shuffle_form = Formation::new((22, 1), Alignment::Percent(0.315, 0.95));
        let shuffle_rect = shuffle_form.rect(frame.area());

        frame.render_widget(hit, hit_rect);
        frame.render_widget(double, double_rect);
        frame.render_widget(split, split_rect);
        frame.render_widget(stand, stand_rect);
        frame.render_widget(quit, quit_rect);
        frame.render_widget(shuffle, shuffle_rect);
    }

    fn handle(&mut self) -> std::io::Result<()> {
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


fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Spades | Suit::Clubs => Color::White,
        Suit::Hearts | Suit::Diamonds => Color::Red,
    }
}