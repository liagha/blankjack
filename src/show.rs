use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use crate::card::{Card, Hand, Suit};

const CARD_WIDTH: u16 = 15;
const CARD_HEIGHT: u16 = 9;
const CARD_GAP: u16 = 2;

pub fn render_card_at(frame: &mut Frame, card: &Card, area: Rect) {
    let style = Style::default().fg(suit_color(card.suit));
    let width = area.width as usize;
    let height = area.height as usize;
    let mut lines = Vec::new();

    for row in 0..height {
        if row == 0 {
            lines.push(Line::from(Span::styled(
                format!(" {}", card.value),
                style,
            )));
        } else if row == height - 3 {
            lines.push(Line::from(Span::styled(
                format!("{}{}", " ".repeat(width - 4), card.value),
                style,
            )));
        } else if row == height / 2 - 1 {
            lines.push(Line::from(Span::styled(
                format!("{}{}", " ".repeat(width / 2 - 1), card.suit),
                style,
            )));
        } else {
            lines.push(Line::from(Span::raw("")));
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

    frame.render_widget(paragraph, area);
}

pub fn render_hidden_card(frame: &mut Frame, area: Rect) {
    let width = area.width as usize;
    let height = area.height as usize;
    let mut lines = Vec::new();

    for row in 0..height {
        if row == 0 || row == height - 1 || row == height / 2 {
            lines.push(Line::from(Span::styled(
                "?".repeat(width.saturating_sub(2)),
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            lines.push(Line::from(Span::raw("")));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(paragraph, area);
}

pub fn render_hand(frame: &mut Frame, hand: &Hand, area: Rect, hide_second: bool) {
    let count = hand.cards.len();
    if count == 0 {
        return;
    }

    let total_card_width = count as u16 * CARD_WIDTH + (count as u16 - 1) * CARD_GAP;
    let start_x = if total_card_width < area.width {
        area.x + (area.width - total_card_width) / 2
    } else {
        area.x
    };

    let y = area.y + (area.height.saturating_sub(CARD_HEIGHT)) / 2;

    for (i, card) in hand.cards.iter().enumerate() {
        let x = start_x + i as u16 * (CARD_WIDTH + CARD_GAP);
        let card_rect = Rect::new(x, y, CARD_WIDTH, CARD_HEIGHT);

        if hide_second && i == 1 {
            render_hidden_card(frame, card_rect);
        } else {
            render_card_at(frame, card, card_rect);
        }
    }
}

pub fn render_sum(frame: &mut Frame, sum: usize, area: Rect) {
    let text = Line::raw(format!("Sum: {}", sum))
        .style(Style::default().fg(Color::White));
    frame.render_widget(text, area);
}

pub fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Spades | Suit::Clubs => Color::White,
        Suit::Hearts | Suit::Diamonds => Color::Red,
    }
}