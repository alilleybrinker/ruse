//! A wrapper around Vec that provides only push and pop.

/// A wrapper around Vec that provides only push and pop.
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Get an empty Stack.
    pub fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    /// Push a new element onto the Stack.
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Pop the top element off of the Stack.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}
