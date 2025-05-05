mod byte_io;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, Weak};
use std::{fmt::Debug, net::TcpStream};

use byte_io::{InputMemoryStream, OutputMemoryStream, ReadStream, Readable, Writable, WriteStream};

pub trait GameObject: Sync + Send + Debug {}

#[derive(Debug, Default)]
pub struct LinkingContext {
    next_id: usize,
    id_to_go: HashMap<usize, Arc<dyn GameObject>>,
    go_to_id: HashMap<usize, usize>,
}

impl LinkingContext {
    pub fn get_network_id(
        &mut self,
        go: &Arc<dyn GameObject>,
        should_create: bool,
    ) -> Option<usize> {
        let data_ptr: *const dyn GameObject = &**go;
        let thin_ptr = data_ptr as *const () as usize;

        match self.go_to_id.entry(thin_ptr) {
            Entry::Occupied(occupied_entry) => Some(*occupied_entry.get()),
            Entry::Vacant(vacant_entry) if should_create => {
                let id = self.next_id;
                self.next_id += 1;
                self.id_to_go.insert(id, go.clone());
                vacant_entry.insert(id);
                Some(id)
            }
            _ => None,
        }
    }

    pub fn get_game_object(&self, id: usize) -> Option<Arc<dyn GameObject>> {
        self.id_to_go.get(&id).cloned()
    }

    pub fn insert_game_object(&mut self, go: Arc<dyn GameObject>, id: usize) {
        let data_ptr: *const dyn GameObject = &*go;
        let thin_ptr = data_ptr as *const () as usize;

        self.id_to_go.insert(id, go);
        self.go_to_id.insert(thin_ptr, id);
    }

    pub fn remove_game_object(&mut self, go: Arc<dyn GameObject>) {
        let data_ptr: *const dyn GameObject = &*go;
        let thin_ptr = data_ptr as *const () as usize;

        let id = self.go_to_id.get(&thin_ptr).cloned().unwrap();
        self.id_to_go.remove(&id);
        self.go_to_id.remove(&thin_ptr);
    }
}

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

impl Writable for RoboCat {
    type Ctx = LinkingContext;

    fn write<W: WriteStream>(&self, stream: &mut W, ctx: &mut Self::Ctx) -> Result<(), W::Error> {
        self.health.write(stream, ctx)?;
        self.meow_count.write(stream, ctx)?;
        self.mice_indices.write(stream, ctx)?;
        self.name.write(stream, ctx)?;
        self.home.write(stream, ctx)?;

        Ok(())
    }
}

impl Readable for RoboCat {
    type Ctx = LinkingContext;

    fn read<R: ReadStream>(stream: &mut R, ctx: &mut Self::Ctx) -> Result<Self, R::Error> {
        Ok(Self {
            health: stream.read_u32()?,
            meow_count: stream.read_u32()?,
            mice_indices: Vec::<u32>::read(stream, ctx)?,
            name: String::read(stream, ctx)?,
            home: Option::<Weak<dyn GameObject>>::read(stream, ctx)?,
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
        let mut output = OutputMemoryStream::new();
        let (mut stream, _) = listener.accept().unwrap();

        rcv.recv().unwrap();

        cat.write(&mut output, &mut *ctx1.lock().unwrap()).unwrap();
        stream.write(output.buffer()).unwrap();
        stream.flush().unwrap();
    });

    let mut client = TcpStream::connect("127.0.0.1:55555").unwrap();

    sdr.send(()).unwrap();

    thread.join().unwrap();

    let mut input = InputMemoryStream::owned();
    client.read(input.buffer_mut()).unwrap();
    let cat = RoboCat::read(&mut input, &mut *ctx.lock().unwrap()).unwrap();

    dbg!(cat);
}
