use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub trait Play {
    fn handle(&mut self, key: KeyEvent);
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn active(&self) -> bool;
}
