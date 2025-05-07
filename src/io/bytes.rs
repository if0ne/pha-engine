use std::{mem::MaybeUninit, string::FromUtf8Error};

pub trait ErasedWriteStream {
    type Error;

    fn write_any(&mut self, v: &[u8]) -> Result<(), Self::Error>;
}

pub trait WriteStream: ErasedWriteStream {
    fn write_bool(&mut self, v: bool) -> Result<(), Self::Error>;

    fn write_u8(&mut self, v: u8) -> Result<(), Self::Error>;
    fn write_u16(&mut self, v: u16) -> Result<(), Self::Error>;
    fn write_u32(&mut self, v: u32) -> Result<(), Self::Error>;
    fn write_u64(&mut self, v: u64) -> Result<(), Self::Error>;

    fn write_i8(&mut self, v: i8) -> Result<(), Self::Error>;
    fn write_i16(&mut self, v: i16) -> Result<(), Self::Error>;
    fn write_i32(&mut self, v: i32) -> Result<(), Self::Error>;
    fn write_i64(&mut self, v: i64) -> Result<(), Self::Error>;

    fn write_usize(&mut self, v: usize) -> Result<(), Self::Error>;
    fn write_isize(&mut self, v: isize) -> Result<(), Self::Error>;

    fn write_f32(&mut self, v: f32) -> Result<(), Self::Error>;
    fn write_f64(&mut self, v: f64) -> Result<(), Self::Error>;
}

pub trait ErasedReadStream {
    type Error: From<FromUtf8Error>;

    fn read_any(&mut self, v: &mut [u8]) -> Result<(), Self::Error>;
}

pub trait ReadStream: ErasedReadStream {
    fn read_bool(&mut self) -> Result<bool, Self::Error>;

    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_u16(&mut self) -> Result<u16, Self::Error>;
    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_u64(&mut self) -> Result<u64, Self::Error>;

    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    fn read_i16(&mut self) -> Result<i16, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;
    fn read_i64(&mut self) -> Result<i64, Self::Error>;

    fn read_usize(&mut self) -> Result<usize, Self::Error>;
    fn read_isize(&mut self) -> Result<isize, Self::Error>;

    fn read_f32(&mut self) -> Result<f32, Self::Error>;
    fn read_f64(&mut self) -> Result<f64, Self::Error>;
}

pub trait Readable<R: ReadStream>: Sized {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error>;
}

pub trait Writable<W: WriteStream> {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error>;
}

impl<W: WriteStream> Writable<W> for bool {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_bool(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for u8 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_u8(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for u16 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_u16(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for u32 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_u32(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for u64 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_u64(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for i8 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_i8(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for i16 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_i16(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for i32 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_i32(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for i64 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_i64(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for usize {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_usize(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for isize {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_isize(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for f32 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_f32(*self)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for f64 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_f64(*self)?;

        Ok(())
    }
}

impl<W: WriteStream, T: Writable<W>> Writable<W> for Option<T> {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        if let Some(v) = self {
            stream.write_bool(true)?;
            v.write_byte(stream)?;
        } else {
            stream.write_bool(false)?;
        }

        Ok(())
    }
}

impl<W: WriteStream, T: Writable<W>, const N: usize> Writable<W> for [T; N] {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        for el in self.iter() {
            el.write_byte(stream)?;
        }

        Ok(())
    }
}

impl<W: WriteStream, T: Writable<W>> Writable<W> for &[T] {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_usize(self.len())?;

        for el in self.iter() {
            el.write_byte(stream)?;
        }

        Ok(())
    }
}

impl<W: WriteStream, T: Writable<W>> Writable<W> for Vec<T> {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        stream.write_usize(self.len())?;

        for el in self {
            el.write_byte(stream)?;
        }

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for &str {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        let bytes = self.as_bytes();
        bytes.write_byte(stream)
    }
}

impl<W: WriteStream> Writable<W> for String {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        let bytes = self.as_bytes();
        bytes.write_byte(stream)
    }
}

impl<W: WriteStream> Writable<W> for glam::Vec2 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        self.x.write_byte(stream)?;
        self.y.write_byte(stream)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for glam::Vec3 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        self.x.write_byte(stream)?;
        self.y.write_byte(stream)?;
        self.z.write_byte(stream)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for glam::Vec4 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        self.x.write_byte(stream)?;
        self.y.write_byte(stream)?;
        self.z.write_byte(stream)?;
        self.w.write_byte(stream)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for glam::Mat2 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        self.x_axis.write_byte(stream)?;
        self.y_axis.write_byte(stream)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for glam::Mat3 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        self.x_axis.write_byte(stream)?;
        self.y_axis.write_byte(stream)?;
        self.z_axis.write_byte(stream)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for glam::Mat4 {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        self.x_axis.write_byte(stream)?;
        self.y_axis.write_byte(stream)?;
        self.z_axis.write_byte(stream)?;
        self.w_axis.write_byte(stream)?;

        Ok(())
    }
}

impl<W: WriteStream> Writable<W> for glam::Quat {
    fn write_byte(&self, stream: &mut W) -> Result<(), W::Error> {
        self.to_array().write_byte(stream)?;

        Ok(())
    }
}

impl<R: ReadStream> Readable<R> for bool {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_bool()
    }
}

impl<R: ReadStream> Readable<R> for u8 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_u8()
    }
}

impl<R: ReadStream> Readable<R> for u16 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_u16()
    }
}

impl<R: ReadStream> Readable<R> for u32 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_u32()
    }
}

impl<R: ReadStream> Readable<R> for u64 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_u64()
    }
}

impl<R: ReadStream> Readable<R> for i8 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_i8()
    }
}

impl<R: ReadStream> Readable<R> for i16 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_i16()
    }
}

impl<R: ReadStream> Readable<R> for i32 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_i32()
    }
}

impl<R: ReadStream> Readable<R> for i64 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_i64()
    }
}

impl<R: ReadStream> Readable<R> for usize {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_usize()
    }
}

impl<R: ReadStream> Readable<R> for isize {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_isize()
    }
}

impl<R: ReadStream> Readable<R> for f32 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_f32()
    }
}

impl<R: ReadStream> Readable<R> for f64 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        stream.read_f64()
    }
}

impl<R: ReadStream, T: Readable<R>> Readable<R> for Option<T> {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        let is_some = stream.read_bool()?;
        if is_some {
            Ok(Some(T::read_byte(stream)?))
        } else {
            Ok(None)
        }
    }
}

impl<R: ReadStream, T: Readable<R>> Readable<R> for Vec<T> {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        let len = stream.read_usize()?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::read_byte(stream)?);
        }
        Ok(vec)
    }
}

impl<R: ReadStream, T: Readable<R>, const N: usize> Readable<R> for [T; N] {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        std::array::try_from_fn(|_| T::read_byte(stream))
    }
}

impl<R: ReadStream> Readable<R> for String {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        let bytes = Vec::<u8>::read_byte(stream)?;
        String::from_utf8(bytes).map_err(|e| e.into())
    }
}

impl<R: ReadStream> Readable<R> for glam::Vec2 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        Ok(Self {
            x: f32::read_byte(stream)?,
            y: f32::read_byte(stream)?,
        })
    }
}

impl<R: ReadStream> Readable<R> for glam::Vec3 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        Ok(Self {
            x: f32::read_byte(stream)?,
            y: f32::read_byte(stream)?,
            z: f32::read_byte(stream)?,
        })
    }
}

impl<R: ReadStream> Readable<R> for glam::Vec4 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        let array = <_>::read_byte(stream)?;
        Ok(Self::from_array(array))
    }
}

impl<R: ReadStream> Readable<R> for glam::Mat2 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        let array = <_>::read_byte(stream)?;
        Ok(Self::from_cols_array(&array))
    }
}

impl<R: ReadStream> Readable<R> for glam::Mat3 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        Ok(Self {
            x_axis: glam::Vec3::read_byte(stream)?,
            y_axis: glam::Vec3::read_byte(stream)?,
            z_axis: glam::Vec3::read_byte(stream)?,
        })
    }
}

impl<R: ReadStream> Readable<R> for glam::Mat4 {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        Ok(Self {
            x_axis: glam::Vec4::read_byte(stream)?,
            y_axis: glam::Vec4::read_byte(stream)?,
            z_axis: glam::Vec4::read_byte(stream)?,
            w_axis: glam::Vec4::read_byte(stream)?,
        })
    }
}

impl<R: ReadStream> Readable<R> for glam::Quat {
    fn read_byte(stream: &mut R) -> Result<Self, R::Error> {
        let array = <_>::read_byte(stream)?;
        Ok(Self::from_array(array))
    }
}

impl<T: ErasedWriteStream> WriteStream for T {
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

impl<T: ErasedReadStream> ReadStream for T {
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
