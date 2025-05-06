use crate::io::bits::{ErasedReadBitStream, ErasedWriteBitStream};

use super::{GameIoError, InputMemoryStream, OutputMemoryStream};

impl<'ctx, 'buffer, T> ErasedWriteBitStream for OutputMemoryStream<'ctx, 'buffer, T> {
    type Error = GameIoError;

    fn write_byte_bits(&mut self, v: u8, bits: usize) -> Result<(), Self::Error> {
        let new_head = self.head + bits;
        let byte_diff = new_head >> 3 - self.buffer.len();

        self.buffer
            .try_reserve(byte_diff)
            .map_err(|_| GameIoError::Oom)?;

        if byte_diff > 0 {
            self.buffer.push(0);
        }

        let byte_offset = self.head >> 3;
        let bit_offset = self.head & 0x7;
        let mask = !(0xFFu8 << bit_offset);
        self.buffer[byte_offset] = (self.buffer[byte_offset] & mask) | (v << bit_offset);
        let bits_free_this_byte = 8 - bit_offset;

        if bits_free_this_byte < bits {
            self.buffer[byte_offset + 1] = v >> bits_free_this_byte;
        }

        self.head = new_head;

        Ok(())
    }

    fn write_any_bits(&mut self, v: &[u8], mut bits: usize) -> Result<(), Self::Error> {
        let mut idx = 0;

        while bits > 8 {
            self.write_byte_bits(v[idx], 8)?;
            idx += 1;
            bits -= 8;
        }

        if bits > 0 {
            self.write_byte_bits(v[idx], bits)?;
        }

        Ok(())
    }
}

impl<'ctx, 'buffer, T> ErasedReadBitStream for InputMemoryStream<'ctx, 'buffer, T> {
    type Error = GameIoError;

    fn read_byte_bit(&mut self, bits: usize) -> Result<u8, Self::Error> {
        let byte_offset = self.head >> 3;
        let bit_offset = self.head & 0x7;

        let mut out = self.buffer[byte_offset] >> bit_offset;

        let bits_free_this_byte = 8 - bit_offset;
        if bits_free_this_byte < bits {
            out |= self.buffer[byte_offset + 1] << bits_free_this_byte;
        }

        out &= !0x00FFu64.wrapping_shl(bits as u32) as u8;

        self.head += bits;

        Ok(out)
    }

    fn read_any_bits(&mut self, v: &mut [u8], mut bits: usize) -> Result<(), Self::Error> {
        let mut idx = 0;

        while bits > 8 {
            v[idx] = self.read_byte_bit(8)?;
            idx += 1;
            bits -= 8;
        }

        if bits > 0 {
            v[idx] = self.read_byte_bit(bits)?;
        }

        Ok(())
    }
}
