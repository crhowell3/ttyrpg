use crate::app::AppResult;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: std::thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if let Ok(poll_result) = event::poll(timeout) {
                        if poll_result {
                            if let Ok(event) = event::read() {
                                let send_result = match event {
                                    CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                                    CrosstermEvent::Resize(w, h) => {
                                        sender.send(Event::Resize(w, h))
                                    }
                                    _ => Ok(()),
                                };
                                if send_result.is_err() {
                                    break;
                                }
                            }
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if sender.send(Event::Tick).is_err() {
                            break;
                        }

                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
