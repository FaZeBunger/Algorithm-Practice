pub struct Item<T> {
    value: T,
    next: Option<Box<Item<T>>>,
}

impl<T> Item<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

pub struct Stack<T> {
    top: Option<Item<T>>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { top: None, size: 0 }
    }

    pub fn push(&mut self, item: Item<T>) {
        item.next = Some(Box::new(self.top.unwrap()));
        self.top = Some(item);
        self.size += 1;
    }

    pub fn add_new(&mut self, value: T) {
        let item = Item::new(value);
        self.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size <= 0 {
            return None;
        }
        let value = self.top.as_ref().unwrap().value;
        self.top = Some(*self.top.unwrap().next.unwrap());
        self.size -= 1;
        Some(value)
    }

    pub fn peek(&self) -> &T {
        &self.top.as_ref().unwrap().value
    }
}
