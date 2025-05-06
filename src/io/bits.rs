use std::string::FromUtf8Error;

pub trait ErasedWriteBitStream {
    type Error;

    fn write_any_bits(&mut self, v: &[u8], bits: usize) -> Result<(), Self::Error>;
}

pub trait WriteBitStream: ErasedWriteBitStream {
    fn write_bool_bits(&mut self, v: bool, bits: usize) -> Result<(), Self::Error>;

    fn write_u8_bits(&mut self, v: u8, bits: usize) -> Result<(), Self::Error>;
    fn write_u16_bits(&mut self, v: u16, bits: usize) -> Result<(), Self::Error>;
    fn write_u32_bits(&mut self, v: u32, bits: usize) -> Result<(), Self::Error>;
    fn write_u64_bits(&mut self, v: u64, bits: usize) -> Result<(), Self::Error>;

    fn write_i8_bits(&mut self, v: i8, bits: usize) -> Result<(), Self::Error>;
    fn write_i16_bits(&mut self, v: i16, bits: usize) -> Result<(), Self::Error>;
    fn write_i32_bits(&mut self, v: i32, bits: usize) -> Result<(), Self::Error>;
    fn write_i64_bits(&mut self, v: i64, bits: usize) -> Result<(), Self::Error>;

    fn write_usize_bits(&mut self, v: usize, bits: usize) -> Result<(), Self::Error>;
    fn write_isize_bits(&mut self, v: isize, bits: usize) -> Result<(), Self::Error>;

    fn write_f32_bits(&mut self, v: f32, bits: usize) -> Result<(), Self::Error>;
    fn write_f64_bits(&mut self, v: f64, bits: usize) -> Result<(), Self::Error>;
}

pub trait ErasedReadBitStream {
    type Error: From<FromUtf8Error>;

    fn read_any_bits(&mut self, v: &mut [u8], bits: usize) -> Result<(), Self::Error>;
}

pub trait ReadBitStream: ErasedReadBitStream {
    fn read_bool_bits(&mut self, bits: usize) -> Result<bool, Self::Error>;

    fn read_u8_bits(&mut self, bits: usize) -> Result<u8, Self::Error>;
    fn read_u16_bits(&mut self, bits: usize) -> Result<u16, Self::Error>;
    fn read_u32_bits(&mut self, bits: usize) -> Result<u32, Self::Error>;
    fn read_u64_bits(&mut self, bits: usize) -> Result<u64, Self::Error>;

    fn read_i8_bits(&mut self, bits: usize) -> Result<i8, Self::Error>;
    fn read_i16_bits(&mut self, bits: usize) -> Result<i16, Self::Error>;
    fn read_i32_bits(&mut self, bits: usize) -> Result<i32, Self::Error>;
    fn read_i64_bits(&mut self, bits: usize) -> Result<i64, Self::Error>;

    fn read_usize_bits(&mut self, bits: usize) -> Result<usize, Self::Error>;
    fn read_isize_bits(&mut self, bits: usize) -> Result<isize, Self::Error>;

    fn read_f32_bits(&mut self, bits: usize) -> Result<f32, Self::Error>;
    fn read_f64_bits(&mut self, bits: usize) -> Result<f64, Self::Error>;
}

pub trait BitReadable<R: ReadBitStream>: Sized {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error>;
}

pub trait BitWritable<W: WriteBitStream> {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error>;
}

impl<W: WriteBitStream> BitWritable<W> for bool {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_bool_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for u8 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_u8_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for u16 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_u16_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for u32 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_u32_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for u64 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_u64_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for i8 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_i8_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for i16 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_i16_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for i32 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_i32_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for i64 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_i64_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for usize {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_usize_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for isize {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_isize_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for f32 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_f32_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for f64 {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_f64_bits(*self, bits)?;

        Ok(())
    }
}

impl<W: WriteBitStream, T: BitWritable<W>> BitWritable<W> for Option<T> {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        if let Some(v) = self {
            stream.write_bool_bits(true, 1)?;
            v.write_bits(stream, bits)?;
        } else {
            stream.write_bool_bits(false, 1)?;
        }

        Ok(())
    }
}

impl<W: WriteBitStream, T: BitWritable<W>> BitWritable<W> for &[T] {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_usize_bits(self.len(), size_of::<usize>() * 8)?;

        for el in self.iter() {
            el.write_bits(stream, bits)?;
        }

        Ok(())
    }
}

impl<W: WriteBitStream, T: BitWritable<W>> BitWritable<W> for Vec<T> {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        stream.write_usize_bits(self.len(), size_of::<usize>() * 8)?;

        for el in self {
            el.write_bits(stream, bits)?;
        }

        Ok(())
    }
}

impl<W: WriteBitStream> BitWritable<W> for &str {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        let bytes = self.as_bytes();
        bytes.write_bits(stream, bits)
    }
}

impl<W: WriteBitStream> BitWritable<W> for String {
    fn write_bits(&self, stream: &mut W, bits: usize) -> Result<(), W::Error> {
        let bytes = self.as_bytes();
        bytes.write_bits(stream, bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for bool {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_bool_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for u8 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_u8_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for u16 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_u16_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for u32 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_u32_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for u64 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_u64_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for i8 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_i8_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for i16 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_i16_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for i32 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_i32_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for i64 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_i64_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for usize {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_usize_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for isize {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_isize_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for f32 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_f32_bits(bits)
    }
}

impl<R: ReadBitStream> BitReadable<R> for f64 {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        stream.read_f64_bits(bits)
    }
}

impl<R: ReadBitStream, T: BitReadable<R>> BitReadable<R> for Option<T> {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        let is_some = stream.read_bool_bits(1)?;
        if is_some {
            Ok(Some(T::read_bits(stream, bits)?))
        } else {
            Ok(None)
        }
    }
}

impl<R: ReadBitStream, T: BitReadable<R>> BitReadable<R> for Vec<T> {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        let len = stream.read_usize_bits(size_of::<usize>() * 8)?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::read_bits(stream, bits)?);
        }
        Ok(vec)
    }
}

impl<R: ReadBitStream> BitReadable<R> for String {
    fn read_bits(stream: &mut R, bits: usize) -> Result<Self, R::Error> {
        let bytes = Vec::<u8>::read_bits(stream, bits)?;
        String::from_utf8(bytes).map_err(|e| e.into())
    }
}
