#[derive(Debug, Clone)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { elements: vec![] }
    }
}

impl<T> Stack<T> {
    pub fn push(&mut self, element: T) {
        self.elements.push(element)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn top(&self) -> Option<&T> {
        if !self.elements.is_empty() {
            Some(&self.elements[self.elements.len() - 1])
        } else {
            None
        }
    }
}
