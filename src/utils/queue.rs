struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn push(&mut self, element: T)  {
        self.data.push(element);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            None
        } else {
            let element = self.data.remove(self.data.len()-1);
            Some(element)
        }
    }
}