use std::sync::Arc;

use tui::layout::{Layout, Direction, Constraint, Rect};

pub struct Scaffold {
    pub output: Rect,
    pub registers: Rect,
    pub code: Rect,
    pub sound_timer: Rect,
    pub delay_timer: Rect,
    pub command: Rect,
    pub arrows: Rect
}

impl Scaffold {
    pub fn new(rect: Rect) -> Self {
        let main_structure = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Min(2),
                    Constraint::Length(5),
                ]
                .as_ref(),
            )
            .split(rect);

        let middle = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Length(5),
                    Constraint::Min(10)
                ].as_ref(),
            )
            .split(main_structure[1]);
        let timers_layout = Scaffold::build_timers_layout(middle[2]);
        
        Self { 
            output: main_structure[0], 
            registers: middle[0], 
            code: middle[2], 
            sound_timer: timers_layout[0], 
            delay_timer: timers_layout[1], 
            command: main_structure[2],
            arrows: middle[1]
        }
    }

    fn build_timers_layout(from: Rect) -> Vec<Rect> {
        let first_half = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref(),
            )
            .split(from);
        let timer_layout: Vec<Rect> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref(),
            )
            .split(first_half[1]);
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref(),
            )
            .split(timer_layout[1])
    }
}