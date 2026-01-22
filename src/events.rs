use std::collections::VecDeque;

#[derive(Debug)]
pub enum AppEvent {
    NES(crate::NESEvent),
    ResetData(ResetTarget),
    Exit,
}

#[derive(Debug)]
pub enum ResetTarget {
    Everything,
    Egui,
}

pub struct AppEventQueue {
    queue: VecDeque<AppEvent>,
}

impl AppEventQueue {
    pub fn new() -> Self {
        AppEventQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, event: AppEvent) {
        self.queue.push_back(event);
    }

    pub fn pull(&mut self) -> Option<AppEvent> {
        self.queue.pop_front()
    }
}
