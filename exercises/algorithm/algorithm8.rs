/*
    queue
    This question requires you to use queues to implement the functionality of the stack
*/
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    // 使用更清晰的命名：main_queue 和 temp_queue
    main_queue: Queue<T>,
    temp_queue: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            main_queue: Queue::<T>::new(),
            temp_queue: Queue::<T>::new(),
        }
    }
    
    pub fn push(&mut self, elem: T) {
        // 方法1：你的实现（每次push都重新排列）
        self.temp_queue.enqueue(elem);
        
        // 将主队列中的所有元素转移到临时队列
        while let Ok(value) = self.main_queue.dequeue() {
            self.temp_queue.enqueue(value);
        }
        
        // 交换两个队列
        std::mem::swap(&mut self.main_queue, &mut self.temp_queue);
    }
    
    // 替代实现：更高效的push方法（可选）
    pub fn push_optimized(&mut self, elem: T) {
        self.main_queue.enqueue(elem);
        let size = self.main_queue.size();
        
        // 将前size-1个元素移到队列末尾
        for _ in 0..size - 1 {
            if let Ok(value) = self.main_queue.dequeue() {
                self.main_queue.enqueue(value);
            }
        }
    }
    
    pub fn pop(&mut self) -> Result<T, &str> {
        self.main_queue.dequeue()
    }
    
    pub fn peek(&self) -> Result<&T, &str> {
        self.main_queue.peek()
    }
    
    pub fn is_empty(&self) -> bool {
        self.main_queue.is_empty()
    }
    
    pub fn size(&self) -> usize {
        self.main_queue.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack_basic() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Queue is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Queue is empty"));
        assert_eq!(s.is_empty(), true);
    }
    
    #[test]
    fn test_stack_peek() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.peek(), Err("Queue is empty"));
        s.push(10);
        assert_eq!(s.peek(), Ok(&10));
        s.push(20);
        assert_eq!(s.peek(), Ok(&20));
        s.pop();
        assert_eq!(s.peek(), Ok(&10));
    }
    
    #[test]
    fn test_stack_size() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.size(), 0);
        s.push(1);
        s.push(2);
        assert_eq!(s.size(), 2);
        s.pop();
        assert_eq!(s.size(), 1);
    }
}