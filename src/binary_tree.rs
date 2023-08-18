#[derive(Debug, Clone)]
pub struct Node<T: Clone> {
    pub value: Option<T>,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct BinaryTree<T: Clone> {
    pub root: Node<T>,
}

#[allow(dead_code)]
impl<T: Clone + std::fmt::Debug> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            root: Node::new(None),
        }
    }
}

#[allow(dead_code)]
impl<T: Clone + std::fmt::Debug> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn add_left(&mut self, value: T) -> &mut Node<T> {
        self.left = Some(Box::new(Node::new(Some(value))));
        self.left.as_mut().unwrap()
    }

    pub fn add_right(&mut self, value: T) -> &mut Node<T> {
        self.right = Some(Box::new(Node::new(Some(value))));
        self.right.as_mut().unwrap()
    }

    pub fn dfs(&self) {
        // Performs a depth-first traversal of the tree
        // and adds the value of each node to a vector
        let left = self.left.as_ref().unwrap();
        let right = self.right.as_ref().unwrap();

        if left.value.is_some() {
            println!("{:?}", left.value);
        }
        if right.value.is_some() {
            println!("{:?}", right.value);
        }

        if left.left.is_some() || left.right.is_some() {
            left.as_ref().dfs();
        }

        if right.left.is_some() || right.right.is_some() {
            right.as_ref().dfs();
        }
    }
}
