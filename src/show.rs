use crate::card::{Card, Hand, Suit};
use ratatui::{
    layout::Rect,
    prelude::{Color, Line, Span, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn render_card(frame: &mut Frame, card: &Card, area: Rect) {
    let style = Style::default().fg(color(card.suit));
    let width = area.width.saturating_sub(2) as usize;
    let height = area.height.saturating_sub(2) as usize;
    let mut lines = Vec::new();

    let val = format!("{}", card.value);
    let suit = format!("{}", card.suit);

    for row in 0..height {
        if row == 0 {
            lines.push(Line::from(Span::styled(format!(" {}", val), style)));
        } else if row == height / 2 && height > 2 {
            let pad = " ".repeat(width.saturating_sub(suit.len()) / 2);
            lines.push(Line::from(Span::styled(format!("{}{}", pad, suit), style)));
        } else if row == height.saturating_sub(1) && height > 3 {
            let pad = " ".repeat(width.saturating_sub(val.len() + 1));
            lines.push(Line::from(Span::styled(format!("{}{} ", pad, val), style)));
        } else {
            lines.push(Line::from(Span::raw("")));
        }
    }

    let item = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(Clear, area);
    frame.render_widget(item, area);
}

pub fn render_hidden(frame: &mut Frame, area: Rect) {
    let width = area.width.saturating_sub(2) as usize;
    let height = area.height.saturating_sub(2) as usize;
    let mut lines = Vec::new();

    for row in 0..height {
        if row == height / 2 {
            let pad = "?".repeat(width);
            lines.push(Line::from(Span::styled(pad, Style::default().fg(Color::DarkGray))));
        } else {
            lines.push(Line::from(Span::raw("")));
        }
    }

    let item = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(Clear, area);
    frame.render_widget(item, area);
}

pub fn render_hand(frame: &mut Frame, hand: &Hand, area: Rect, hide: bool) {
    let count = hand.cards.len() as u16;
    if count == 0 {
        return;
    }

    let gap = 1;
    let width = 10.min(area.width.saturating_div(count).saturating_sub(gap).max(4));
    let height = 7.min(area.height);

    let total = count * width + count.saturating_sub(1) * gap;
    let start = area.x + area.width.saturating_sub(total) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;

    for (i, card) in hand.cards.iter().enumerate() {
        let offset = i as u16 * (width + gap);
        if start + offset + width > area.x + area.width {
            break;
        }

        let item = Rect::new(start + offset, y, width, height);

        if hide && i == 1 {
            render_hidden(frame, item);
        } else {
            render_card(frame, card, item);
        }
    }
}

pub fn render_sum(frame: &mut Frame, label: &str, sum: usize, area: Rect) {
    let text = Line::raw(format!("{}: {}", label, sum)).style(Style::default().fg(Color::White));
    frame.render_widget(Clear, area);
    frame.render_widget(text, area);
}

pub fn color(suit: Suit) -> Color {
    match suit {
        Suit::Spades | Suit::Clubs => Color::White,
        Suit::Hearts | Suit::Diamonds => Color::Red,
    }
}
