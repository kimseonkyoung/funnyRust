use std::collections::VecDeque;

fn main() {

    let mut stack: Vec<i32> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("push 후: {:?}", stack);

    //Some, None 
    while let Some(val) = stack.pop() {
        println!("pop: {}",  val);
    }

    stack.push(10);
    stack.push(20);

    if let Some(top) = stack.last() {
        println!("peak: {}", top);
    }
    
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    println!("enqueue 후: {:?}", queue);

    while let Some(val) = queue.pop_front() {
        print!("dequeue: {}", val);
    }

    queue.push_back(100);
    queue.push_back(200);
    if let Some(front) = queue.front() {
        print!("peak: {}", front);
    }

    print!("peek 후 큐 유지: {:?}", queue);

    
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_stack_push_pop() {
        let mut stack: Vec<i32> = Vec::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2)); //LIFO check
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack: Vec<i32> = Vec::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.last(), Some(&20)); 
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_queue_fifo() {
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(1);
        queue.push_back(2);
        queue.push_back(3);
        assert_eq!(queue.pop_front(), Some(1));
        assert_eq!(queue.pop_front(), Some(2));
        assert_eq!(queue.pop_front(), Some(3));
        assert_eq!(queue.pop_front(), None);
    }

    #[test]
    fn test_queue_peek() {
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(100);
        queue.push_back(200);
        assert_eq!(queue.front(), Some(&100)); 
        assert_eq!(queue.len(), 2);
    }
}