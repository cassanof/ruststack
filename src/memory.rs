use std::cell::RefCell;

use crate::cpu::CpuError;

/// Represents a memory buffer, where the data is stored in 8 bit cells. Supports a maximum size of
/// (2^16)-1 cells.
pub struct Memory {
    memory: RefCell<Vec<u8>>,
}

pub(crate) trait InspectableAddr {
    type Error;
    /// Inspects a place in memory at the given address, returns 8 bytes of data starting from that
    /// place.
    fn inspect_addr(&self, addr: u16) -> Result<String, Self::Error>;
}

impl Memory {
    pub fn new(size: u16) -> Memory {
        Memory {
            memory: RefCell::new(vec![0; size as usize]),
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

    /// Gets the size of the memory buffer.
    pub fn len(&self) -> usize {
        self.memory.borrow().len()
    }

    #[must_use]
    /// Determines if the memory is empty or not.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl InspectableAddr for Memory {
    type Error = CpuError;

    fn inspect_addr(&self, addr: u16) -> Result<String, CpuError> {
        if self.memory.borrow().len() < addr as usize {
            return Err(CpuError::InvalidAddress(addr));
        }
        let end = {
            if (addr as usize) + 8 > self.memory.borrow().len() {
                self.memory.borrow().len()
            } else {
                addr as usize + 8
            }
        };
        let bytes = self.memory.borrow()[addr as usize..end]
            .iter()
            .fold(String::new(), |acc, b| format!("{} 0x{:02X}", acc, b));
        Ok(format!("0x{:04X}:{}", addr, bytes))
    }
}
