use crate::io::bits::{ErasedReadBitStream, ErasedWriteBitStream, ReadBitStream, WriteBitStream};

use super::{GameIoError, InputMemoryStream, OutputMemoryStream};

impl<'ctx, 'buffer, T> ErasedWriteBitStream for OutputMemoryStream<'ctx, 'buffer, T> {
    type Error = GameIoError;

    fn write_any_bits(&mut self, v: &[u8], mut bits: usize) -> Result<(), Self::Error> {
        let mut idx = 0;

        while bits > 8 {
            self.write_u8_bits(v[idx], 8)?;
            idx += 1;
            bits -= 8;
        }

        if bits > 0 {
            self.write_u8_bits(v[idx], bits)?;
        }

        Ok(())
    }
}

impl<'ctx, 'buffer, T> WriteBitStream for OutputMemoryStream<'ctx, 'buffer, T> {
    fn write_bool_bits(&mut self, v: bool, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&(v as u8).to_le_bytes(), bits)
    }

    fn write_u8_bits(&mut self, v: u8, bits: usize) -> Result<(), Self::Error> {
        let next_bit_head = self.head + bits;
        self.buffer
            .try_reserve(bits / 8 + (bits % 8 == 0) as usize)
            .map_err(|_| GameIoError::Oom)?;

        if self.buffer.len() * 8 - self.head < bits {
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

        self.head = next_bit_head;

        Ok(())
    }

    fn write_u16_bits(&mut self, v: u16, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_u32_bits(&mut self, v: u32, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_u64_bits(&mut self, v: u64, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_i8_bits(&mut self, v: i8, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_i16_bits(&mut self, v: i16, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_i32_bits(&mut self, v: i32, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_i64_bits(&mut self, v: i64, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_usize_bits(&mut self, v: usize, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_isize_bits(&mut self, v: isize, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_le_bytes(), bits)
    }

    fn write_f32_bits(&mut self, v: f32, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_bits().to_le_bytes(), bits)
    }

    fn write_f64_bits(&mut self, v: f64, bits: usize) -> Result<(), Self::Error> {
        self.write_any_bits(&v.to_bits().to_le_bytes(), bits)
    }
}

impl<'ctx, 'buffer, T> ErasedReadBitStream for InputMemoryStream<'ctx, 'buffer, T> {
    type Error = GameIoError;

    fn read_any_bits(&mut self, v: &mut [u8], mut bits: usize) -> Result<(), Self::Error> {
        let mut idx = 0;

        while bits > 8 {
            v[idx] = self.read_u8_bits(8)?;
            idx += 1;
            bits -= 8;
        }

        if bits > 0 {
            v[idx] = self.read_u8_bits(bits)?;
        }

        Ok(())
    }
}

impl<'ctx, 'buffer, T> ReadBitStream for InputMemoryStream<'ctx, 'buffer, T> {
    fn read_bool_bits(&mut self, bits: usize) -> Result<bool, Self::Error> {
        let mut v = [0u8; size_of::<bool>()];
        self.read_any_bits(&mut v, bits)?;
        Ok(v[0] > 0)
    }

    fn read_u8_bits(&mut self, bits: usize) -> Result<u8, Self::Error> {
        let byte_offset = self.head >> 3;
        let bit_offset = self.head & 0x7;

        let mut out = self.buffer[byte_offset] >> bit_offset;

        let bits_free_this_byte = 8 - bit_offset;
        if bits_free_this_byte < bits {
            out |= self.buffer[byte_offset + 1] << bits_free_this_byte;
        }

        out &= !0x00FFu32.wrapping_shl(bits as u32) as u8;

        self.head += bits;

        Ok(out)
    }

    fn read_u16_bits(&mut self, bits: usize) -> Result<u16, Self::Error> {
        let mut v = [0u8; size_of::<u16>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(u16::from_le_bytes(v))
    }

    fn read_u32_bits(&mut self, bits: usize) -> Result<u32, Self::Error> {
        let mut v = [0u8; size_of::<u32>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(u32::from_le_bytes(v))
    }

    fn read_u64_bits(&mut self, bits: usize) -> Result<u64, Self::Error> {
        let mut v = [0u8; size_of::<u64>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(u64::from_le_bytes(v))
    }

    fn read_i8_bits(&mut self, bits: usize) -> Result<i8, Self::Error> {
        let mut v = [0u8; size_of::<i8>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(i8::from_le_bytes(v))
    }

    fn read_i16_bits(&mut self, bits: usize) -> Result<i16, Self::Error> {
        let mut v = [0u8; size_of::<i16>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(i16::from_le_bytes(v))
    }

    fn read_i32_bits(&mut self, bits: usize) -> Result<i32, Self::Error> {
        let mut v = [0u8; size_of::<i32>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(i32::from_le_bytes(v))
    }

    fn read_i64_bits(&mut self, bits: usize) -> Result<i64, Self::Error> {
        let mut v = [0u8; size_of::<i64>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(i64::from_le_bytes(v))
    }

    fn read_usize_bits(&mut self, bits: usize) -> Result<usize, Self::Error> {
        let mut v = [0u8; size_of::<usize>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(usize::from_le_bytes(v))
    }

    fn read_isize_bits(&mut self, bits: usize) -> Result<isize, Self::Error> {
        let mut v = [0u8; size_of::<isize>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(isize::from_le_bytes(v))
    }

    fn read_f32_bits(&mut self, bits: usize) -> Result<f32, Self::Error> {
        let mut v = [0u8; size_of::<f32>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(f32::from_le_bytes(v))
    }

    fn read_f64_bits(&mut self, bits: usize) -> Result<f64, Self::Error> {
        let mut v = [0u8; size_of::<f64>()];
        self.read_any_bits(&mut v, bits)?;

        Ok(f64::from_le_bytes(v))
    }
}
