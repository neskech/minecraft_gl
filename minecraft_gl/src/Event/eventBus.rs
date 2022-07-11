use queues::*;
use super::event::Event;

pub struct EventBus{
    Bus: Queue<Event>,
}

impl EventBus{
    pub fn New() -> Self {
        Self { Bus: Queue::new() }
    }

    pub fn AddEvent(&mut self, ev: Event){
        self.Bus.add(ev).unwrap();
    }

    pub fn Remove(&mut self) -> Event{
        self.Bus.remove().unwrap()
    }

    pub fn Size(&self) -> usize{
        self.Bus.size()
    }
}