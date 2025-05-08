pub mod bits;
pub mod bytes;

use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
pub enum GameIoError {
    Utf8Error(FromUtf8Error),
    UnregisteredGameObject(usize),
    UnexpectedEof(usize, usize),
    Oom,
}

impl From<FromUtf8Error> for GameIoError {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

pub struct OutputMemoryStream<'ctx, 'buffer, T> {
    pub ctx: &'ctx mut T,

    buffer: &'buffer mut Vec<u8>,
    head: usize,
}

impl<'ctx, 'buffer, T> OutputMemoryStream<'ctx, 'buffer, T> {
    pub fn new(buffer: &'buffer mut Vec<u8>, ctx: &'ctx mut T) -> Self {
        Self {
            buffer,
            head: 0,
            ctx,
        }
    }
}

pub struct InputMemoryStream<'ctx, 'buffer, T> {
    pub ctx: &'ctx mut T,

    buffer: &'buffer [u8],
    head: usize,
}

impl<'ctx, 'buffer, T> InputMemoryStream<'ctx, 'buffer, T> {
    pub fn new(buffer: &'buffer [u8], ctx: &'ctx mut T) -> Self {
        Self {
            buffer,
            head: 0,
            ctx,
        }
    }

    pub fn remaining_bit_count(&self) -> usize {
        self.buffer.len() * 8 - self.head
    }
}
