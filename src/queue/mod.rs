use std::panic::RefUnwindSafe;

pub struct Queue {
  older: Vec<char>,
  yonger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c:char) {
      self.yonger.push(c);
    }

    pub fn pop(&mut self,) -> Option<char> {
      if self.older.is_empty() {

        if self.yonger.is_empty() {
          return  None;
        }

        use std::mem::swap;
        swap(&mut self.older, &mut self.yonger);
        self.older.reverse();
      }

      self.older.pop()
    }

    pub fn create() -> Queue {
      Queue { older: Vec::new(), yonger: Vec::new(), }
    }
}