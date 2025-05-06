use crate::io::{
    bits::{ErasedReadBitStream, ErasedWriteBitStream},
    bytes::{ErasedReadStream, ErasedWriteStream},
};

use super::{GameIoError, InputMemoryStream, OutputMemoryStream};

impl<'ctx, 'buffer, T> ErasedWriteStream for OutputMemoryStream<'ctx, 'buffer, T> {
    type Error = GameIoError;

    fn write_any(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        if self.head % 8 == 0 {
            self.buffer
                .try_reserve(v.len())
                .map_err(|_| GameIoError::Oom)?;
            self.buffer.extend_from_slice(v);
            self.head += v.len() * 8;

            Ok(())
        } else {
            self.write_any_bits(v, v.len() * 8)
        }
    }
}

impl<T> ErasedReadStream for InputMemoryStream<'_, '_, T> {
    type Error = GameIoError;

    fn read_any(&mut self, v: &mut [u8]) -> Result<(), Self::Error> {
        if self.head < (self.buffer.len() + v.len()) * 8 {
            if self.head % 8 == 0 {
                v.copy_from_slice(&self.buffer[(self.head / 8)..(self.head / 8 + v.len())]);
                self.head += v.len() * 8;
                return Ok(());
            } else {
                return self.read_any_bits(v, v.len() * 8);
            }
        }

        Err(GameIoError::UnexpectedEof(
            v.len(),
            self.buffer.len() - self.head,
        ))
    }
}
