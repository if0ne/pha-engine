use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, Weak};
use std::{fmt::Debug, net::TcpStream};

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

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut()
    }
}

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

    fn buffer(&self) -> &[u8];

    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_i8(&mut self) -> Result<i8, Self::Error>;

    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;

    fn read_usize(&mut self) -> Result<usize, Self::Error>;
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

    fn buffer(&self) -> &[u8] {
        &self.buffer
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
        stream.write(&output.buffer).unwrap();
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
