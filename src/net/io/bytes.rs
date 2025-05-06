use crate::io::{
    bits::{ErasedReadBitStream, ErasedWriteBitStream},
    bytes::{ErasedReadStream, ErasedWriteStream, ReadStream, WriteStream},
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
            self.write_any_bits(v, 8)
        }
    }
}

impl<'ctx, 'buffer, T> WriteStream for OutputMemoryStream<'ctx, 'buffer, T> {
    fn write_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.write_any(&(v as u8).to_le_bytes())
    }

    fn write_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        self.write_any(&v.to_le_bytes())
    }

    fn write_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.write_any(&v.to_bits().to_le_bytes())
    }

    fn write_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        self.write_any(&v.to_bits().to_le_bytes())
    }
}

impl<'ctx, 'buffer, T> ErasedReadStream for InputMemoryStream<'ctx, 'buffer, T> {
    type Error = GameIoError;

    fn read_any(&mut self, v: &mut [u8]) -> Result<(), Self::Error> {
        if self.head < (self.buffer.len() + v.len()) * 8 {
            if self.head % 8 == 0 {
                v.copy_from_slice(&self.buffer[(self.head / 8)..(self.head / 8 + v.len())]);
                self.head += v.len() * 8;
                return Ok(());
            } else {
                return self.read_any_bits(v, 8);
            }
        }

        return Err(GameIoError::UnexpectedEof(
            v.len(),
            self.buffer.len() - self.head,
        ));
    }
}

impl<'ctx, 'buffer, T> ReadStream for InputMemoryStream<'ctx, 'buffer, T> {
    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        let mut v = [0u8; size_of::<bool>()];
        self.read_any(&mut v)?;
        Ok(v[0] > 0)
    }

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let mut v = [0u8; size_of::<u8>()];
        self.read_any(&mut v)?;

        Ok(u8::from_le_bytes(v))
    }

    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        let mut v = [0u8; size_of::<u16>()];
        self.read_any(&mut v)?;

        Ok(u16::from_le_bytes(v))
    }

    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        let mut v = [0u8; size_of::<u32>()];
        self.read_any(&mut v)?;

        Ok(u32::from_le_bytes(v))
    }

    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        let mut v = [0u8; size_of::<u64>()];
        self.read_any(&mut v)?;

        Ok(u64::from_le_bytes(v))
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        let mut v = [0u8; size_of::<i8>()];
        self.read_any(&mut v)?;

        Ok(i8::from_le_bytes(v))
    }

    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        let mut v = [0u8; size_of::<i16>()];
        self.read_any(&mut v)?;

        Ok(i16::from_le_bytes(v))
    }

    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        let mut v = [0u8; size_of::<i32>()];
        self.read_any(&mut v)?;

        Ok(i32::from_le_bytes(v))
    }

    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        let mut v = [0u8; size_of::<i64>()];
        self.read_any(&mut v)?;

        Ok(i64::from_le_bytes(v))
    }

    fn read_usize(&mut self) -> Result<usize, Self::Error> {
        let mut v = [0u8; size_of::<usize>()];
        self.read_any(&mut v)?;

        Ok(usize::from_le_bytes(v))
    }

    fn read_isize(&mut self) -> Result<isize, Self::Error> {
        let mut v = [0u8; size_of::<isize>()];
        self.read_any(&mut v)?;

        Ok(isize::from_le_bytes(v))
    }

    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        let mut v = [0u8; size_of::<f32>()];
        self.read_any(&mut v)?;

        Ok(f32::from_le_bytes(v))
    }

    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        let mut v = [0u8; size_of::<f64>()];
        self.read_any(&mut v)?;

        Ok(f64::from_le_bytes(v))
    }
}
