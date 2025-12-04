use itermore::IterNextChunk;

/// Helper methods to get little-end bytes out of a byte iterator
pub trait BytesLE: Iterator<Item = u8> {
    fn next_u8(&mut self) -> Option<u8> {
        self.next().map(u8::from_le)
    }

    // Note: When MS says "WORD", this is what they mean.
    fn next_u16(&mut self) -> Option<u16>
    where
        Self: Sized,
    {
        self.next_array().ok().map(u16::from_le_bytes)
    }

    fn next_u32(&mut self) -> Option<u32>
    where
        Self: Sized,
    {
        self.next_array().ok().map(u32::from_le_bytes)
    }
}

impl<I> BytesLE for I where I: Iterator<Item = u8> {}
