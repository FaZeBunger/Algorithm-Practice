struct LRS<T> {
    pub items: Vec<T>,
    pub max_size: usize,
}

impl<T> LRS<T> {
    fn with_capacity(size: usize) -> Self {
        LRS {
            items: Vec::new(),
            max_size: size,
        }
    }
}
