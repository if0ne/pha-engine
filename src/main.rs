mod game_io;
mod io;
mod linking_context;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, Weak};
use std::{fmt::Debug, net::TcpStream};

use game_io::{GameIoError, InputMemoryStream, OutputMemoryStream};
use io::bytes::{ReadStream, Readable, Writable};
use linking_context::LinkingContext;

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

impl Writable<OutputMemoryStream<'_, '_, LinkingContext>> for RoboCat {
    fn write(
        &self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<(), GameIoError> {
        self.health.write(stream)?;
        self.meow_count.write(stream)?;
        self.mice_indices.write(stream)?;
        self.name.write(stream)?;
        self.home.write(stream)?;

        Ok(())
    }
}

impl Readable<InputMemoryStream<'_, '_, LinkingContext>> for RoboCat {
    fn read(stream: &mut InputMemoryStream<'_, '_, LinkingContext>) -> Result<Self, GameIoError> {
        Ok(Self {
            health: stream.read_u32()?,
            meow_count: stream.read_u32()?,
            mice_indices: Vec::<u32>::read(stream)?,
            name: String::read(stream)?,
            home: Option::<Weak<dyn GameObject>>::read(stream)?,
        })
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

        cat.write(&mut output).unwrap();
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

    let cat = RoboCat::read(&mut input).unwrap();

    dbg!(cat);
}
