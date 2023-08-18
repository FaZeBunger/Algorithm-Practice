pub struct Stack<T> {
    pub size: usize,
    items: Vec<T>,
    pub max_size: Option<usize>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        // Creates a new stack that grows as needed
        Stack {
            size: 0,
            items: Vec::new(),
            max_size: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        // Creates a new stack with a fixed capacity
        Stack {
            size: 0,
            items: Vec::with_capacity(capacity),
            max_size: Some(capacity),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        // Pushes an item onto the stack
        if let Some(max_size) = self.max_size {
            if self.size >= max_size {
                return Err("Stack overflow");
            }
        };
        self.items.push(item);
        self.size += 1;
        Ok(())
    }

    pub fn peek(&self) -> Option<&T> {
        // Returns a reference to the top item on the stack
        self.items.last()
    }

    pub fn pop(&mut self) -> Option<T> {
        // Removes and returns the top item on the stack
        if self.size == 0 {
            return None;
        }
        let out = self.items.pop();
        self.size -= 1;
        out
    }
}
