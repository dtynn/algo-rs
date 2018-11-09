#[derive(Debug)]
pub struct Queue {
    pushes: Vec<i32>,
    pops: Vec<i32>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            pushes: Vec::new(),
            pops: Vec::new(),
        }
    }

    pub fn add(&mut self, n: i32) {
        self.pushes.push(n);
    }

    pub fn poll(&mut self) -> Option<i32> {
        if self.pops.is_empty() {
            while !self.pushes.is_empty() {
                self.pops.push(self.pushes.pop().unwrap());
            }
        }

        self.pops.pop()
    }

    pub fn peek(&self) -> usize {
        self.pushes.len() + self.pops.len()
    }
}
