use std::{ops::Index};
use anyhow::{Result, anyhow};


/// A ring buffer is a buffer that wraps around when the end of the buffer is reached.
/// 
/// # Examples
/// 
/// ```
/// use ggpo_core::structs::RingBuffer;
/// 
/// let mut buffer: RingBuffer<usize, 2> = RingBuffer::new();
/// 
/// buffer.push(1).unwrap();
/// 
/// assert_eq!(buffer.front().unwrap(), &1);
/// ```
pub struct RingBuffer<T, const N: usize> {
    head: usize,
    tail: usize,
    size: usize,
    elements: [T; N],
}

impl<T, const N: usize> RingBuffer<T, N> 
where
    T: Default + Copy
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn front(&self) -> Option<&T> {
        if self.size == N {
            None
        }
        else {
            self.elements.get(self.tail)
        }
    }

    pub fn push(&mut self, element: T) -> Result<()> {
        if self.size == N - 1 {
            return Err(anyhow!("Ring buffer is full"));
        }

        self.elements[self.head] = element;
        self.head = (self.head + 1) % N;
        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<()> {
        if self.size == N {
            return Err(anyhow!("Ring buffer is empty."));
        }
        self.tail = (self.tail + 1) % N;
        self.size -= 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> RingBufferIterator<'_, T, N> {
        RingBufferIterator {
            buffer: self,
            index: 0,
        }
    }

}

impl<T, const N: usize> Index<usize> for RingBuffer<T, N> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        if self.size >= N {

            panic!("Index out of bounds");
        }
        let extracted_index = (self.tail + index) % N;

        self.elements.get(extracted_index).expect(&format!("Couldn't get element at index {}", index))
    }

}




impl<T, const N: usize> Default for RingBuffer<T, N>
where
    T: Default + Copy
{
    fn default() -> Self {
        Self {
            head: 0,
            tail: 0,
            size: 0,
            elements: [T::default(); N],
        }
    }
}

pub struct RingBufferIterator<'a, T, const N: usize> {
    buffer: &'a RingBuffer<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for RingBufferIterator<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.buffer.len() {
            None
        }
        else {
            let element = &self.buffer[self.index];
            self.index += 1;
            Some(element)
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn create_ring_buffer() {
        let buffer: RingBuffer<usize, 2> = RingBuffer::new();
        assert_eq!(buffer[0], 0);
        assert_eq!(*buffer.front().unwrap(), 0);
    }

    #[test]
    fn ring_buffer_iterator() {
        let mut buffer: RingBuffer<usize, 5> = RingBuffer::new();

        buffer.push(4).unwrap();
        buffer.push(3).unwrap();
        buffer.push(2).unwrap();
        buffer.push(1).unwrap();

        let mut iterator = buffer.iter();
        assert_eq!(iterator.next().unwrap(), &4);
        assert_eq!(iterator.next().unwrap(), &3);
        assert_eq!(iterator.next().unwrap(), &2);
        assert_eq!(iterator.next().unwrap(), &1);
    }
}