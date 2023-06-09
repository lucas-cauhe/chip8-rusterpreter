use std::{thread, sync::{Arc, /* Mutex, */ mpsc::{Sender, self}}, time::Duration};
use parking_lot::Mutex;

use chip8::timers::Signals;
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
    pub style: Arc<Mutex<TextArea<'static>>>,
    pub time_left: Arc<Mutex<u32>>,
    pub tx: Sender<Signals>
}
impl DelayTimerComponent {
    pub fn new(count: u32) -> Self {
        let mut delay_timer = TextArea::new(["Delay Timer is on".to_string(), "Real time can't be display as of now".to_string(), "".to_string()].to_vec());
        delay_timer.set_style(Style::default().fg(Color::Green));
        delay_timer.set_alignment(Alignment::Left);
        delay_timer.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("delay Timer Simulation")
                    .border_type(BorderType::Plain),
            );
        let timer = Arc::new(Mutex::new(count));
        let thread_timer = Arc::clone(&timer);
        let arc_st = Arc::new(Mutex::new(delay_timer));
        let arc_st_clone = Arc::clone(&arc_st);
        // launch thread to update this timer component
        let tx = launch_timer_thread(thread_timer, arc_st_clone);
        Self { 
            style: arc_st,
            time_left: timer,
            tx
        }
    }
}

pub struct SoundTimerComponent {
    pub style: Arc<Mutex<TextArea<'static>>>,
    pub time_left: Arc<Mutex<u32>>,
    pub tx: Sender<Signals>
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
        let tx = launch_timer_thread(thread_timer, arc_st_clone);
        Self { 
            style: arc_st,
            time_left: timer,
            tx
        }
    }
}

fn launch_timer_thread(thread_timer: Arc<Mutex<u32>>, arc_st: Arc<Mutex<TextArea<'static>>>) -> Sender<Signals> {
    let (tx, rx) = mpsc::channel::<Signals>();
    thread::spawn( move || {
        loop {
            // simulate timer
            // decrement timer with freq 1Hz
            match rx.try_recv() {
                Ok(sig) => {
                    match sig {
                        Signals::KILL => break,
                        Signals::RES => { },
                        Signals::STP => {rx.recv().unwrap();}
                    }
                },
                _ => { }
            }
            let mut t_lck = thread_timer.lock();
            *t_lck -= 1;
            if *t_lck <= 0 {
                break;
            }
            drop(t_lck);
            let mut lck = arc_st.lock();
            lck.delete_line_by_head(); 
            lck.insert_str(thread_timer.lock().to_string());
            drop(lck);
            thread::sleep(Duration::new(1, 0));
        }
    });
    tx
}