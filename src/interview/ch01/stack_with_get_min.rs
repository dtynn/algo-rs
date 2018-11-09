#[derive(Debug)]
pub struct StackWithGetMin {
    elements: Vec<i32>,
    mins: Vec<i32>,
}

impl StackWithGetMin {
    pub fn push(&mut self, n: i32) {
        self.elements.push(n);
        if self.mins.is_empty() {
            self.mins.push(n);
        } else {
            let min = self.mins.last().unwrap().clone();
            self.mins.push({
                if min < n {
                    min
                } else {
                    n
                }
            });
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.elements.pop() {
            Some(num) => {
                self.mins.pop();
                return Some(num);
            }

            None => return None,
        }
    }

    pub fn get_min(&self) -> Option<i32> {
        match self.mins.last() {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }
}
