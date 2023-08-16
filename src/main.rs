// mod binary_tree;
mod stack;

pub use stack::Stack;

#[cfg(test)]
mod tests {
    fn binary_tree_tests() {}

    #[test]
    fn stack_tests() {
        use crate::stack::Stack;

        let mut stack = Stack::new();
        stack.add_new(1);
        stack.add_new(2);
        stack.add_new(3);
        stack.add_new(4);
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.peek(), &3);
        assert_eq!(stack.pop(), Some(3));
        stack.add_new(5);
        assert_eq!(stack.peek(), &5);
    }
}

fn main() {}
