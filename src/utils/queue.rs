use std::collections::VecDeque;

pub struct Queue {
    data: VecDeque<Box<dyn FnOnce() + Send>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue { data: VecDeque::new() }
    }

    pub fn push<F>(&mut self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.data.push_back(Box::new(task));
    }

    pub fn pop(&mut self) -> Option<Box<dyn FnOnce() + Send>> {
        self.data.pop_front()
    }
}
