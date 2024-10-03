pub(crate) struct Queue {
    current_processing: bool,
    data: Vec<Box<dyn FnOnce() + Send>>, //pointer into heap space with Box
}

impl Queue {
    pub(crate) fn new() -> Self {
        Queue { current_processing: false, data: Vec::new() }
    }

    // Push a callable task into the queue
    pub(crate) fn push<F>(&mut self, task: F)
    where//good to go deep into type f_parameters description
        F: FnOnce() + Send + 'static,
    {
        self.data.push(Box::new(task));
        self.processing_queue()
    }

    fn pop(&mut self) -> Option<Box<dyn FnOnce() + Send>> {
        if self.data.is_empty() {
            None
        } else {
            //FIFO
            Some(self.data.remove(0))
        }
    }

    fn processing_queue(&mut self) {
        if !self.current_processing {
            self.current_processing = true;

            while let Some(task) = self.pop() {
                //  callable task
                task();
            }

            self.current_processing = false;
        }
    }
}