use color_eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    // event sender channel
    sender: mpsc::Sender<Event>,
    // event receiver channel
    receiver: mpsc::Receiver<Event>,
    // event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate: Duration = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender: mpsc::Sender<Event> = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout).expect("Unable to poll for event") {
                        match event::read().expect("Unable to read event") {
                            CrosstermEvent::Key(key) => {
                                if key.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(key))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on Windows
                                }
                            }
                            CrosstermEvent::Mouse(mouse) => {
                                sender.send(Event::Mouse(mouse))
                            }
                            CrosstermEvent::Resize(width, height) => {
                                sender.send(Event::Resize(width, height))
                            }
                            _ => unimplemented!(),
                        }.expect("failed to send event")
                    }
                    if last_tick.elapsed() >= tick_rate { 
                        sender.send(Event::Tick).expect("failed to send tick event");
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

    pub fn next(&self) -> Result<Event> { 
        Ok(self.receiver.recv()?)
    }
}
