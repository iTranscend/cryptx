pub struct LoopingIter<T> {
    index: usize,
    buffer: Vec<T>,
}

impl<T> LoopingIter<T> {
    pub fn new(buffer: Vec<T>) -> Self {
        Self { 
            index: 0,
            buffer: buffer,
        }
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.index >= self.buffer.len() {
            self.index = 0;
        }
        let item = self.buffer.get(self.index);
        self.index += 1;
        item
    }
}