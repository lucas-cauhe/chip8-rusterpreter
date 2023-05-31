use std::{thread, sync::{Arc, Mutex, mpsc::{self, Sender, TryRecvError}}};
use std::time::{Duration};

use crate::chip8::RTI_DEFAULT_ADDR;

#[derive(Debug)]
pub struct TimerThread {
    pub timer: u8,
    // whats the purpose of the timer
    // holds the direction a subroutine to handle the timer event
    // it can either be custom-made or by default it should send some kind of signal to a pause-like instruction
    pub rti: u16
}

impl TimerThread {
    pub fn launch(count: u8, rti: Option<u16>) -> (Arc<Mutex<Self>>, Sender<()>) {
        let new_timer = Arc::new(Mutex::new(TimerThread { 
            timer: count, 
            rti: match rti {
                Some(addr) => addr,
                None => RTI_DEFAULT_ADDR
            }
        }));
        let new_timer_clone = Arc::clone(&new_timer);
        let (tx, rx) = mpsc::channel::<()>();
        thread::Builder::new().name("timer_thread".to_string()).spawn(move || {
            loop { 
                thread::sleep(Duration::new(0, 16_666_667));
                match rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => {
                        debug_assert!(false, "Kill message received or sender disconnected");
                        break
                    },
                    Err(TryRecvError::Empty) => {}
                }
                let mut t = new_timer_clone.lock().unwrap();
                t.timer -= 1;
                if t.timer == 0 {
                    break;
                }
            }
        }).unwrap();
        (new_timer, tx)
    }
}


#[cfg(test)]
mod tests {
    use std::{thread, time::Duration, sync::Arc};

    use super::TimerThread;
    #[test]
    fn timer_setup() {
        let (timer, _s) = TimerThread::launch(10, None);
        let mut times = [0; 5];
        for i in 0..5 {
            thread::sleep(Duration::new(0, 40_000_000));
            let timer_lck = timer.lock().unwrap();
            times[i] = timer_lck.timer;
        }
        for i in 1..5 {
            assert!(times[i] < times[i-1], "Failed at: {:}, {:}", times[i], times[i-1]);
        }
    }

    #[test]
    fn timer_kill() {
        let (timer, s) = TimerThread::launch(10, None);
        s.send(()).unwrap();
        // let scheduler select the other thread
        thread::sleep(Duration::new(0,50_000_000));
        assert_eq!(Arc::strong_count(&timer), 1, "There are more threads than expected: {}", Arc::strong_count(&timer));
    }
}

