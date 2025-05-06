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
        let new_head = self.head.checked_add(v.len() * 8).ok_or_else(|| {
            GameIoError::UnexpectedEof(v.len() * 8, self.buffer.len() * 8 - self.head)
        })?;

        if self.head < new_head {
            if self.head % 8 == 0 {
                let begin = self.head >> 3;
                v.copy_from_slice(&self.buffer[begin..(begin + v.len())]);
                self.head = new_head;
                Ok(())
            } else {
                self.read_any_bits(v, v.len() * 8)
            }
        } else {
            Err(GameIoError::UnexpectedEof(
                v.len() * 8,
                self.buffer.len() * 8 - self.head,
            ))
        }
    }
}
