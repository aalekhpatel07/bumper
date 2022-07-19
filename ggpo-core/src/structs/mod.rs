#[cfg(not(feature = "ringbuf"))]
mod ring_buffer;

#[cfg(not(feature = "ringbuf"))]
pub use ring_buffer::*;


#[cfg(feature = "ringbuf")]
pub use ringbuf::*;