use std::sync::{Arc, Weak};

use crate::{GameObject, LinkingContext};

pub trait WriteStream {
    type Error;

    fn buffer(&self) -> &[u8];

    fn write_u8(&mut self, v: u8) -> Result<(), Self::Error>;
    fn write_i8(&mut self, v: i8) -> Result<(), Self::Error>;

    fn write_u32(&mut self, v: u32) -> Result<(), Self::Error>;
    fn write_i32(&mut self, v: i32) -> Result<(), Self::Error>;

    fn write_usize(&mut self, v: usize) -> Result<(), Self::Error>;
}

pub trait ReadStream {
    type Error;

    fn buffer_mut(&mut self) -> &mut [u8];

    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_i8(&mut self) -> Result<i8, Self::Error>;

    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;

    fn read_usize(&mut self) -> Result<usize, Self::Error>;
}

pub struct OutputMemoryStream {
    buffer: Vec<u8>,
}

impl OutputMemoryStream {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(32),
        }
    }
}

pub struct InputMemoryStream {
    buffer: Vec<u8>,
    head: usize,
}

impl InputMemoryStream {
    pub fn owned() -> Self {
        Self {
            buffer: vec![0u8; 1470],
            head: 0,
        }
    }
}

impl WriteStream for OutputMemoryStream {
    type Error = std::io::Error;

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn write_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.buffer.extend(&v.to_le_bytes());

        Ok(())
    }

    fn write_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        self.buffer.extend(&v.to_le_bytes());

        Ok(())
    }

    fn write_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.buffer.extend(&v.to_le_bytes());

        Ok(())
    }

    fn write_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.buffer.extend(&v.to_le_bytes());

        Ok(())
    }

    fn write_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        self.buffer.extend(&v.to_le_bytes());

        Ok(())
    }
}

impl ReadStream for InputMemoryStream {
    type Error = std::io::Error;

    fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let size = size_of::<u8>();
        let v = u8::from_le_bytes(
            self.buffer[self.head..(self.head + size)]
                .try_into()
                .unwrap(),
        );
        self.head += size;

        Ok(v)
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        let size = size_of::<i8>();
        let v = i8::from_le_bytes(
            self.buffer[self.head..(self.head + size)]
                .try_into()
                .unwrap(),
        );
        self.head += size;

        Ok(v)
    }

    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        let size = size_of::<u32>();
        let v = u32::from_le_bytes(
            self.buffer[self.head..(self.head + size)]
                .try_into()
                .unwrap(),
        );
        self.head += size;

        Ok(v)
    }

    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        let size = size_of::<i32>();
        let v = i32::from_le_bytes(
            self.buffer[self.head..(self.head + size)]
                .try_into()
                .unwrap(),
        );
        self.head += size;

        Ok(v)
    }

    fn read_usize(&mut self) -> Result<usize, Self::Error> {
        let size = size_of::<usize>();
        let v = usize::from_le_bytes(
            self.buffer[self.head..(self.head + size)]
                .try_into()
                .unwrap(),
        );
        self.head += size;

        Ok(v)
    }
}

pub trait Readable: Sized {
    type Ctx;

    fn read<R: ReadStream>(stream: &mut R, ctx: &mut Self::Ctx) -> Result<Self, R::Error>;
}

pub trait Writable {
    type Ctx;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error>;
}

impl Writable for u8 {
    type Ctx = LinkingContext;

    fn write<W: WriteStream>(&self, stream: &mut W, _: &mut Self::Ctx) -> Result<(), W::Error> {
        stream.write_u8(*self)?;

        Ok(())
    }
}

impl Readable for u8 {
    type Ctx = LinkingContext;

    fn read<R: ReadStream>(stream: &mut R, _: &mut Self::Ctx) -> Result<Self, R::Error> {
        stream.read_u8()
    }
}

impl Writable for u32 {
    type Ctx = LinkingContext;

    fn write<W: WriteStream>(&self, stream: &mut W, _: &mut Self::Ctx) -> Result<(), W::Error> {
        stream.write_u32(*self)?;

        Ok(())
    }
}

impl Readable for u32 {
    type Ctx = LinkingContext;

    fn read<R: ReadStream>(stream: &mut R, _: &mut Self::Ctx) -> Result<Self, R::Error> {
        stream.read_u32()
    }
}

impl<T: Writable> Writable for &[T] {
    type Ctx = T::Ctx;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error> {
        stream.write_usize(self.len())?;

        for el in self.iter() {
            el.write(stream, ctx)?;
        }

        Ok(())
    }
}

impl<T: Writable> Writable for Vec<T> {
    type Ctx = T::Ctx;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error> {
        self.as_slice().write(stream, ctx)
    }
}

impl<T: Readable> Readable for Vec<T> {
    type Ctx = T::Ctx;

    fn read<R: ReadStream>(stream: &mut R, ctx: &mut Self::Ctx) -> Result<Self, R::Error> {
        let count = stream.read_usize()?;
        let mut vec = Vec::with_capacity(count);

        for _ in 0..count {
            vec.push(T::read(stream, ctx)?);
        }

        Ok(vec)
    }
}

impl Writable for String {
    type Ctx = LinkingContext;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error> {
        self.as_bytes().write(stream, ctx)
    }
}

impl Readable for String {
    type Ctx = LinkingContext;

    fn read<R: ReadStream>(stream: &mut R, ctx: &mut Self::Ctx) -> Result<Self, R::Error> {
        let vec = Vec::<u8>::read(stream, ctx)?;

        Ok(String::from_utf8(vec).unwrap())
    }
}

impl Writable for Option<Weak<dyn GameObject>> {
    type Ctx = LinkingContext;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error> {
        self.as_ref().and_then(|x| x.upgrade()).write(stream, ctx)
    }
}

impl Writable for Arc<dyn GameObject> {
    type Ctx = LinkingContext;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error> {
        stream.write_usize(ctx.get_network_id(self, false).unwrap())?;

        Ok(())
    }
}

impl Readable for Arc<dyn GameObject> {
    type Ctx = LinkingContext;

    fn read<R: ReadStream>(stream: &mut R, ctx: &mut Self::Ctx) -> Result<Self, R::Error> {
        let go = ctx.get_game_object(stream.read_usize()?).unwrap();

        Ok(go)
    }
}

impl Readable for Option<Weak<dyn GameObject>> {
    type Ctx = LinkingContext;

    fn read<R: ReadStream>(stream: &mut R, ctx: &mut Self::Ctx) -> Result<Self, R::Error> {
        if stream.read_u8()? == 0 {
            return Ok(None);
        }

        let go = ctx.get_game_object(stream.read_usize()?).unwrap();
        Ok(Some(Arc::downgrade(&go)))
    }
}

impl<T: Writable> Writable for Option<T> {
    type Ctx = T::Ctx;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error> {
        match self {
            Some(v) => {
                stream.write_u8(1)?;
                v.write(stream, ctx)
            }
            None => stream.write_u8(0),
        }
    }
}
