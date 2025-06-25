use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use crate::card::{Card, Deck};
use crate::{suit_color};

pub struct Formation {
    size: (u16, u16),
    alignment: Alignment,
}

impl Default for Formation {
    fn default() -> Formation {
        Formation {
            size: (0, 0),
            alignment: Alignment::TopLeft,
        }
    }    
}

impl Formation {
    pub fn new(size: (u16, u16), alignment: Alignment) -> Self {
        Formation {
            size,
            alignment,
        }
    }
    
    pub fn rect(self, parent: Rect) -> Rect {
        let (width, height) = (self.size.0, self.size.1);

        let x = match self.alignment {
            Alignment::Left => 0,
            Alignment::Center => (parent.width.saturating_sub(width)) / 2,
            Alignment::Right => parent.width.saturating_sub(width),
            Alignment::TopLeft | Alignment::Top => 0,
            Alignment::TopRight => parent.width.saturating_sub(width),
            Alignment::BottomLeft | Alignment::Bottom => 0,
            Alignment::BottomRight => parent.width.saturating_sub(width),
            Alignment::Custom(x, _) => x,
            Alignment::Percent(px, _) => ((parent.width as f32 - width as f32) * px) as u16,
        };

        let y = match self.alignment {
            Alignment::Top | Alignment::TopLeft | Alignment::TopRight => 0,
            Alignment::Left | Alignment::Right | Alignment::Center => (parent.height.saturating_sub(height)) / 2,
            Alignment::Bottom | Alignment::BottomLeft | Alignment::BottomRight => parent.height.saturating_sub(height),
            Alignment::Custom(_, y) => y,
            Alignment::Percent(_, py) => ((parent.height as f32 - height as f32) * py) as u16,
        };

        Rect::new(parent.x + x, parent.y + y, width, height)
    }
}

pub enum Alignment {
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

pub trait Show {
    fn render(&self, frame: &mut Frame, formation: Formation);
}

impl Show for Deck {
    fn render(&self, frame: &mut Frame, formation: Formation) {
        for (index, card) in self.cards.iter().enumerate() {
            let alignment = Alignment::Percent((index + 1) as f32 * 0.075, 0.25);
            let formation = Formation::new((15, 9), alignment);
            card.render(frame, formation);
        }
    }
}

impl Show for Card {
    fn render(&self, frame: &mut Frame, formation: Formation) {
        let rect = formation.rect(frame.area());
        
        let width = rect.width as usize;
        let height = rect.height as usize;
        let style = Style::default().fg(suit_color(self.suit));
        let mut lines = Vec::new();

        for index in 0..height {
            if index == 0 {
                lines.push(
                    Line::from(
                        Span::styled(
                            format!(" {}", self.value),
                            style,
                        )
                    )
                );
            } else if index == height - 3 {
                lines.push(
                    Line::from(
                        Span::styled(
                            format!("{}{}", " ".repeat(width - 4), self.value),
                            style,
                        )
                    )
                );
            } else if index == height / 2 - 1{
                lines.push(
                    Line::from(
                        Span::styled(
                            format!("{}{}", " ".repeat(width / 2 - 1), self.suit),
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

        frame.render_widget(paragraph, rect);
    }
}