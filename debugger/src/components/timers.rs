use std::{thread, sync::{Arc, Mutex}, time::Duration, borrow::BorrowMut};

use tui::{
    layout::{Alignment, Rect}, 
    widgets::{Paragraph, Borders, BorderType, Block}, 
    style::{Color, Style}
};
use tui_textarea::TextArea;

use crate::display::DefaultTerminal;

// DISCLAIMER

// SIMULATION TIMERS WORK AT A LOWER FREQUENCY THAN CHIP TIMERS SINCE THE DEBUGGER SHOULDN'T MODIFY CHIP'S INTERFACE TO GET THE CURRENT STATE OF REAL TIMERS
// THAT IS, TIMERS DISPLAYED ARE ONLY FOR ILUSTRATION PURPOSES AND SHOW THERE IS A TIMER ON DURING SIMULATION

pub struct DelayTimerComponent {
    pub style: Paragraph<'static>
}
impl DelayTimerComponent {
    pub fn new() -> Self {
        let delay_timer = Paragraph::new("Delay Timer")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Delay Timer")
                    .border_type(BorderType::Plain),
            );
        Self { 
            style: delay_timer
        }
    }
}

pub struct SoundTimerComponent {
    pub style: Arc<Mutex<TextArea<'static>>>,
    pub time_left: Arc<Mutex<u32>>
}
impl SoundTimerComponent {
    pub fn new(count: u32) -> Self {
        let mut sound_timer = TextArea::new(["Sound Timer is on".to_string(), "Real time can't be display as of now".to_string(), "".to_string()].to_vec());
        sound_timer.set_style(Style::default().fg(Color::Green));
        sound_timer.set_alignment(Alignment::Left);
        sound_timer.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Sound Timer Simulation")
                    .border_type(BorderType::Plain),
            );
        let timer = Arc::new(Mutex::new(count));
        let thread_timer = Arc::clone(&timer);
        let arc_st = Arc::new(Mutex::new(sound_timer));
        let arc_st_clone = Arc::clone(&arc_st);
        // launch thread to update this timer component
        thread::spawn( move || {
            loop {
                // simulate timer
                // decrement timer with freq 1Hz
                let mut t_lck = thread_timer.lock().unwrap();
                *t_lck -= 1;
                if *t_lck <= 0 {
                    break;
                }
                drop(t_lck);
                let mut lck = arc_st_clone.lock();
                let st_lck = lck.as_deref_mut().unwrap();
                st_lck.delete_line_by_head(); 
                st_lck.insert_str(thread_timer.lock().unwrap().to_string());
                drop(lck);
                thread::sleep(Duration::new(1, 0));
            }
        });
        Self { 
            style: arc_st,
            time_left: timer
        }
    }
}