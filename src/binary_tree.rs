use std::cell::RefCell;
use std::rc::{self, Rc};

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    pub fn new(item: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode {
            item,
            next: None,
            prev: None,
        }))
    }
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push_back(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }
}

fn main() {}
