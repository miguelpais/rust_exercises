#[derive(Debug, Clone)]
pub struct Stack<T: Clone> {
    top: Option<StackElement<T>>,
    size: u32
}

#[derive(Debug, Clone)]
struct StackElement<T: Clone> {
    value: T,
    next: Option<Box<StackElement<T>>>
}

impl<T: Clone> Stack<T> {
    pub fn empty() -> Stack<T> {
        Stack {
            top: None,
            size: 0
        }
    }

    pub fn from(value: T) -> Stack<T> {
        Stack {
            top: Some(StackElement {
                value,
                next: None
            }),
            size: 1
        }
    }

    pub fn push(mut self, value: T) -> Stack<T> {
        if self.size == 0 {
            self.top = Some(StackElement {
                value,
                next: None
            })
        }
        else {
            let old_top = self.top.clone().unwrap();
            self.top = Some(StackElement {
                value,
                next: Some(Box::new(old_top))
            })
        }
        self.size += 1;
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size <= 0 {
            return None;
        }

        let old_top = self.top.clone().unwrap();
        let popped_value = Some(old_top.value);
        if self.size == 1 {
            self.top = None;
        }
        else {
            let next_top = *old_top.next.clone().unwrap();
            self.top = Some(next_top);
        };

        self.size -= 1;
        popped_value
    }
}
