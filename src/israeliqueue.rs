pub struct IsraeliQueue<I> {
    queue: Vec<I>,
}

impl<I> IsraeliQueue<I> {
    pub fn new() -> IsraeliQueue<I> {
        IsraeliQueue { queue: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> &I {
        &self.queue[0]
    }

    pub fn dequeue(&mut self) -> I {
        self.queue.remove(0)
    }

    pub fn queue(&mut self, item: I, comparator: Option<&Fn(&I, &I) -> bool>) -> usize {
        if comparator.is_none() {
            self.queue.push(item);
            return self.queue.len() - 1;
        }

        let mut found_index = self.queue.len();

        for (index, element) in self.queue.iter().enumerate() {
            if comparator.unwrap()(&item, element) {
                found_index = index;
                break;
            }
        }

        self.queue.insert(found_index, item);
        found_index
    }
}
