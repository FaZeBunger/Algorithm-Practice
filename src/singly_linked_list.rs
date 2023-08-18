use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Node<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Clone, Debug)]
struct Singly_Linked_List<T: Clone> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
}

impl<T: Clone> Singly_Linked_List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, value: T) {
        if let Some(tail) = &mut self.tail {
            tail.borrow_mut().push(value);
        } else {
            self.head = Some(Node::new(value));
            self.tail = self.head.clone();
        }
    }
}

impl<T: Clone> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }

    fn push(&mut self, value: T) {
        self.next = Some(Rc::new(RefCell::new(Node::new(value))));
    }

    fn has_next(&self) -> bool {
        match self.next {
            None => false,
            Some(_) => true,
        }
    }

    fn get_next(&self) -> Option<Ref<Node<T>>> {
        if let Some(next) = &self.next {
            Some(next.borrow())
        } else {
            None
        }
    }
}
