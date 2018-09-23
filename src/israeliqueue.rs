use std::collections::vec_deque::IntoIter;
use std::collections::VecDeque;
use std::iter::IntoIterator;

#[derive(Clone, Default, Debug)]
pub struct IsraeliQueue<I> {
    queue: VecDeque<I>,
}

impl<I> IsraeliQueue<I> {
    pub fn new() -> IsraeliQueue<I> {
        IsraeliQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&I> {
        self.queue.get(0)
    }

    pub fn dequeue(&mut self) -> Option<I> {
        self.queue.pop_front()
    }

    pub fn queue(&mut self, item: I, comparator: Option<&Fn(&I, &I) -> bool>) -> usize {
        if comparator.is_none() {
            self.queue.push_front(item);
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

// Do I also implement referenced iterators? I'll leave it this way for now.
impl<I> IntoIterator for IsraeliQueue<I> {
    type Item = I;
    type IntoIter = IntoIter<I>;

    fn into_iter(self) -> IntoIter<I> {
        self.queue.into_iter()
    }
}
