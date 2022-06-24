#[derive(Default)]
pub struct CircularBuffer<T: Default + Clone> {
    data: Vec<T>,
    write_idx: usize,
    read_idx: usize,
    capacity: usize,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

/*
 * We maintain a write and a read pointer into the buffer. Write
 * is done at the location of the write ptr, and read is done from
 * the location of the read ptr. The pointers are incremented after
 * corresponding read/write operations, modulo to the capacity.
 *
 * The exception to the above is when the buffer is full and an
 * overwrite is done, then we write at the location of the read
 * ptr.
 *
 * Basically, the read ptr points to the oldest element, and the
 * write ptr to the next free slot.
 */
impl<T: Default + Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![Default::default(); capacity],
            capacity,
            ..Default::default()
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.data[self.write_idx] = element;
        self.write_idx = (self.write_idx + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let element = self.data[self.read_idx].clone();
        self.read_idx = (self.read_idx + 1) % self.capacity;
        self.size -= 1;
        Ok(element)
    }

    pub fn clear(&mut self) {
        self.data = vec![Default::default(); self.capacity];
        self.read_idx = 0;
        self.write_idx = 0;
        self.size = 0;
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.data[self.read_idx] = element;
            self.read_idx = (self.read_idx + 1) % self.capacity;
            self.size += 1;
        } else {
            _ = self.write(element);
        }
    }
}
