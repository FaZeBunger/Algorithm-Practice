#[allow(unused_assignments, unused_variables)]
#[allow(dead_code)]
pub enum QueueError {
    QueueFull,
    QueueEmpty,
}

#[allow(dead_code)]
pub struct Queue<T> {
    pub size: usize,
    items: Vec<T>,
    pub max_size: Option<usize>,
}

#[allow(dead_code)]
impl<T> Queue<T> {
    pub fn new() -> Self {
        // Creates a new queue that grows as needed
        Queue {
            size: 0,
            items: Vec::new(),
            max_size: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        // Creates a new queue with a fixed capacity
        Queue {
            size: 0,
            items: Vec::with_capacity(capacity),
            max_size: Some(capacity),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), QueueError> {
        // Pushes an item onto the queue
        if let Some(max_size) = self.max_size {
            if self.size >= max_size {
                return Err(QueueError::QueueFull);
            }
        };
        self.items.push(item);
        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        // Removes and returns the top item on the queue
        if self.size <= 0 {
            return None;
        }
        self.size -= 1;
        Some(self.items.remove(0))
    }

    pub fn peek(&self) -> Option<&T> {
        // Returns a reference to the top item on the queue
        self.items.first()
    }
}
