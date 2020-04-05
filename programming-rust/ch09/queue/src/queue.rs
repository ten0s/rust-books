/// A generic LIFO queue
#[derive(Debug)]
pub struct Queue<T> {
    front: Vec<T>,
    back: Vec<T>,
}

impl<T> Queue<T> {
    /// Constructs a new, empty Queue<T>
    pub fn new() -> Self {
        Queue {
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }

    /// Push a value into the back of a queue.
    pub fn push(&mut self, v: T) {
        self.back.push(v);
    }

    /// Pop a value off the front of a queue.
    /// Return `Some(c)` of there was a value to pop,
    /// or `None` if the queue was empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.front.is_empty() {
            if self.back.is_empty() {
                return None;
            }

            // Swap younger with front and back.
            use std::mem::swap;
            swap(&mut self.front, &mut self.back);
            self.front.reverse();
        }

        self.front.pop()
    }

    /// Split the queue to (front, back)
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.front, self.back)
    }
}
