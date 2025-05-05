use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, Weak};
use std::{fmt::Debug, net::TcpStream};

use zerocopy::{FromBytes, Immutable, IntoBytes};

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

impl RoboCat {
    pub fn write(&self, stream: &mut OutputMemoryStream, ctx: &mut LinkingContext) {
        stream.write(&self.health);
        stream.write(&self.meow_count);
        stream.write(&self.mice_indices.len());
        stream.write_many(self.mice_indices.as_slice());
        stream.write(&self.name.len());
        stream.write_many(&self.name.as_bytes());

        if let Some(home) = self.home.as_ref().and_then(|go| go.upgrade()) {
            stream.write(&1u8);
            stream.write(&ctx.get_network_id(&home, false).unwrap());
        } else {
            stream.write(&0u8);
        }
    }

    pub fn read(&mut self, stream: &mut InputMemoryStream, ctx: &LinkingContext) {
        let mut count = 0usize;
        stream.read(&mut self.health);
        stream.read(&mut self.meow_count);
        stream.read(&mut count);
        self.mice_indices.resize(count, 0);
        stream.read_many(self.mice_indices.as_mut_slice());
        stream.read(&mut count);
        let mut vec = vec![0u8; count];
        stream.read_many(&mut vec);
        self.name = String::from_utf8(vec).unwrap();

        let mut flag = 0u8;
        stream.read(&mut flag);

        if flag != 0 {
            let mut id = 0usize;
            stream.read(&mut id);

            self.home = Some(Arc::downgrade(&ctx.get_game_object(id).unwrap()));
        }
    }
}

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

impl OutputMemoryStream {
    pub fn write<T: IntoBytes + Immutable>(&mut self, data: &T) {
        self.buffer.extend(data.as_bytes());
    }

    pub fn write_many<T: IntoBytes + Immutable>(&mut self, data: &[T]) {
        self.buffer.extend(data.as_bytes());
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

pub struct InputMemoryStream {
    buffer: Vec<u8>,
    head: usize,
}

impl InputMemoryStream {
    pub fn owned() -> Self {
        Self {
            buffer: vec![0u8; 128],
            head: 0,
        }
    }

    pub fn read<T: FromBytes>(&mut self, data: &mut T) {
        let size = size_of::<T>();
        let value = T::read_from_bytes(&self.buffer[self.head..(self.head + size)]).unwrap();
        self.head += size;
        *data = value;
    }

    pub fn read_many<T: FromBytes>(&mut self, data: &mut [T]) {
        let size = size_of::<T>();

        for v in data {
            let value = T::read_from_bytes(&self.buffer[self.head..(self.head + size)]).unwrap();
            self.head += size;
            *v = value;
        }
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut()
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

        cat.write(&mut output, &mut *ctx1.lock().unwrap());
        stream.write(&output.buffer).unwrap();
        stream.flush().unwrap();
    });

    let mut client = TcpStream::connect("127.0.0.1:55555").unwrap();

    sdr.send(()).unwrap();

    thread.join().unwrap();

    let mut input = InputMemoryStream::owned();
    client.read(input.buffer_mut()).unwrap();
    let mut cat = RoboCat::default();
    cat.read(&mut input, &*ctx.lock().unwrap());

    dbg!(cat);
}
