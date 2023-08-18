mod LRS;
mod binary_tree;
mod queue;
mod singly_linked_list;
mod stack;

pub use stack::Stack;

#[cfg(test)]
mod tests {
    #[test]
    fn stack_tests() {
        use crate::stack::Stack;

        let mut stack = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    #[allow(unused_assignments, unused_variables)]
    fn queue_tests() {
        use crate::queue::Queue;

        let mut queue = Queue::new();
        queue.push(1).unwrap_or_else(|_| panic!("Queue full"));
        queue.push(2).unwrap_or_else(|_| panic!("Queue full"));
        queue.push(3).unwrap_or_else(|_| panic!("Queue full"));
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn binary_tree_test() {
        use crate::binary_tree::BinaryTree;

        let tree = BinaryTree::new();
        let mut root = tree.root;

        assert_eq!(root.value, None);
        root.add_left(1);
        root.add_right(2);
        root.left.unwrap().add_left(3);
        root.right.unwrap().add_right(4);
    }
}

fn main() {}
