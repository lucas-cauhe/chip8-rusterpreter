use std::{thread, sync::{Arc, Mutex}};
use std::time::{Duration};



#[derive(Debug)]
pub struct TimerThread {
    pub timer: u16,
    // whats the purpose of the timer
    // holds the direction a subroutine to handle the timer event
    // it can either be custom-made or by default it should send some kind of signal to a pause-like instruction
    // pub setter: 
}

impl TimerThread {
    pub fn launch(count: u16) -> Arc<Mutex<Self>> {
        let new_timer = Arc::new(Mutex::new(TimerThread { timer: count }));
        let new_timer_clone = new_timer.clone();
        thread::spawn(move || {
            loop { 
                thread::sleep(Duration::new(0, 16666667));
                let mut t = new_timer_clone.lock().unwrap();
                t.timer -= 1;
                if t.timer == 0 {
                    break;
                }
            }
        });
        new_timer
    }
}

