#![feature(array_try_from_fn)]

mod io;
mod linking_context;
mod net;
mod reflect;
mod utils;

use std::io::{Read, Write};
use std::mem::offset_of;
use std::net::TcpListener;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::{fmt::Debug, net::TcpStream};

use io::bytes::{Readable, Writable};
use linking_context::LinkingContext;
use net::io::{InputMemoryStream, OutputMemoryStream};
use reflect::{MemberField, Reflect, Ty, UserDefinedType};

pub trait GameObject: Sync + Send + Debug {
    fn id(&self) -> usize;
    fn class_id(&self) -> u32;
}

#[derive(Debug)]
pub struct RoboCat {
    id: usize,
    health: u32,
    meow_count: u32,
    name: String,
}

pub static ID: AtomicUsize = AtomicUsize::new(0);

impl GameObject for RoboCat {
    fn id(&self) -> usize {
        self.id
    }

    fn class_id(&self) -> u32 {
        Self::type_id()
    }
}

impl Default for RoboCat {
    fn default() -> Self {
        Self {
            id: ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            health: 10,
            meow_count: 3,
            name: Default::default(),
        }
    }
}

impl Reflect for RoboCat {
    fn reflect(&self) -> &'static reflect::UserDefinedType {
        const INFO: &'static UserDefinedType = &UserDefinedType::new(&[
            MemberField::new("health", Ty::Int, offset_of!(RoboCat, health)),
            MemberField::new("meow_count", Ty::Int, offset_of!(RoboCat, meow_count)),
            MemberField::new("name", Ty::String, offset_of!(RoboCat, name)),
        ]);

        INFO
    }

    fn type_id() -> u32 {
        1
    }

    fn create_instance() -> Self {
        Default::default()
    }
}

fn main() {
    let ctx = Arc::new(Mutex::new(LinkingContext::default()));

    let mut cat = RoboCat::default();
    cat.name = "Eminem".to_string();

    let listener = TcpListener::bind("127.0.0.1:55555").unwrap();
    let (sdr, rcv) = channel();

    let ctx1 = ctx.clone();
    let thread = std::thread::spawn(move || {
        let mut buf = Vec::with_capacity(128);
        let mut ctx = ctx1.lock().unwrap();
        let mut output = OutputMemoryStream::new(&mut buf, &mut *ctx);
        let (mut stream, _) = listener.accept().unwrap();

        rcv.recv().unwrap();

        cat.write_byte(&mut output).unwrap();
        stream.write(&buf).unwrap();
        stream.flush().unwrap();
    });

    let mut client = TcpStream::connect("127.0.0.1:55555").unwrap();

    sdr.send(()).unwrap();

    thread.join().unwrap();

    let mut recv = vec![0u8; 1470];
    client.read(&mut recv).unwrap();
    let mut ctx = ctx.lock().unwrap();
    let mut input = InputMemoryStream::new(&recv, &mut *ctx);

    let cat = RoboCat::read_byte(&mut input).unwrap();

    dbg!(cat);
}
