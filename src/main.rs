#![feature(array_try_from_fn)]

mod io;
mod linking_context;
mod net;
mod reflect;
mod utils;

use std::io::{Read, Write};
use std::mem::offset_of;
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, Weak};
use std::{fmt::Debug, net::TcpStream};

use io::bytes::{Readable, Writable};
use linking_context::LinkingContext;
use net::io::{InputMemoryStream, OutputMemoryStream};
use reflect::{MemberField, Reflect, Ty, UserDefinedType};

pub trait GameObject: Sync + Send + Debug {}

#[derive(Debug)]
pub struct RoboCat {
    health: u32,
    meow_count: u32,
    home: Option<Weak<dyn GameObject>>,
    name: String,
    mice_indices: Vec<u32>,
}

#[derive(Debug)]
pub struct Home {
    name: String,
    cats: Vec<Arc<dyn GameObject>>,
}

impl Default for Home {
    fn default() -> Self {
        Home {
            name: "Catsville".to_string(),
            cats: Default::default(),
        }
    }
}

impl GameObject for Home {}
impl GameObject for RoboCat {}

impl Default for RoboCat {
    fn default() -> Self {
        Self {
            health: 10,
            meow_count: 3,
            home: None,
            name: Default::default(),
            mice_indices: Default::default(),
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
}

fn main() {
    let ctx = Arc::new(Mutex::new(LinkingContext::default()));

    let home: Arc<dyn GameObject> = Arc::new(Home {
        name: "My Home".to_string(),
        cats: vec![],
    });

    let mut cat = RoboCat::default();
    cat.name = "Eminem".to_string();
    cat.mice_indices.push(4);
    cat.mice_indices.push(2);
    cat.home = Some(Arc::downgrade(&home));

    ctx.lock().unwrap().get_network_id(&home, true);

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
