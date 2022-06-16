use std::cell::RefCell;

/// Represents a memory buffer, where the data is stored in 8 bit cells.
pub struct Memory {
    memory: RefCell<Vec<u8>>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            memory: RefCell::new(vec![0; size]),
        }
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.memory.borrow().get(index).copied()
    }

    /// Gets a slice of the memory buffer, from the given start index to the given end index.
    pub fn get_buf(&self, from: usize, to: usize) -> Option<Vec<u8>> {
        self.memory.borrow().get(from..to).map(|x| x.to_vec())
    }

    /// Sets the value at the given index to the given value.
    pub fn set(&self, index: usize, value: u8) {
        self.memory.borrow_mut()[index] = value;
    }

    /// Sets a slice of the memory buffer, from the given start index to the given end index.
    pub fn set_buf(&self, from: usize, to: usize, value: &[u8]) {
        self.memory.borrow_mut()[from..to].copy_from_slice(value);
    }
}
