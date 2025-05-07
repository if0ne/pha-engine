use std::{
    collections::{HashMap, hash_map::Entry},
    sync::{Arc, Weak},
};

use crate::{
    GameObject,
    io::bytes::{ReadStream, Readable, Writable, WriteStream},
    net::io::{GameIoError, InputMemoryStream, OutputMemoryStream},
    reflect::{Reflect, Ty},
};

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

impl Writable<OutputMemoryStream<'_, '_, LinkingContext>> for Option<Weak<dyn GameObject>> {
    fn write_byte(
        &self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<(), GameIoError> {
        self.as_ref().and_then(|x| x.upgrade()).write_byte(stream)
    }
}

impl Writable<OutputMemoryStream<'_, '_, LinkingContext>> for Arc<dyn GameObject> {
    fn write_byte(
        &self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<(), GameIoError> {
        let id = stream
            .ctx
            .get_network_id(self, false)
            .ok_or(GameIoError::UnregisteredGameObject)?;
        stream.write_usize(id)?;

        Ok(())
    }
}

impl Readable<InputMemoryStream<'_, '_, LinkingContext>> for Arc<dyn GameObject> {
    fn read_byte(
        stream: &mut InputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<Self, GameIoError> {
        let go = stream
            .ctx
            .get_game_object(stream.read_usize()?)
            .ok_or(GameIoError::UnregisteredGameObject)?;

        Ok(go)
    }
}

impl Readable<InputMemoryStream<'_, '_, LinkingContext>> for Option<Weak<dyn GameObject>> {
    fn read_byte(
        stream: &mut InputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<Self, GameIoError> {
        if !stream.read_bool()? {
            return Ok(None);
        }

        let go = stream
            .ctx
            .get_game_object(stream.read_usize()?)
            .ok_or(GameIoError::UnregisteredGameObject)?;

        Ok(Some(Arc::downgrade(&go)))
    }
}

impl<T> Writable<OutputMemoryStream<'_, '_, LinkingContext>> for T
where
    T: Reflect,
{
    fn write_byte(
        &self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<(), GameIoError> {
        unsafe {
            let this = self as *const T as *const u8;

            for field in self.reflect().fields {
                match field.ty {
                    Ty::Int => u32::write_byte(&*(this.add(field.offset) as *const u32), stream)?,
                    Ty::String => {
                        String::write_byte(&*(this.add(field.offset) as *const String), stream)?
                    }
                    Ty::Float => f32::write_byte(&*(this.add(field.offset) as *const f32), stream)?,
                }
            }
        }

        Ok(())
    }
}

impl<T> Readable<InputMemoryStream<'_, '_, LinkingContext>> for T
where
    T: Reflect + Default,
{
    fn read_byte(
        stream: &mut InputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<Self, GameIoError> {
        unsafe {
            let mut ret = T::default();
            let this = &mut ret as *mut T as *mut u8;

            for field in ret.reflect().fields {
                match field.ty {
                    Ty::Int => {
                        *(this.add(field.offset) as *mut u32) = u32::read_byte(stream)?;
                    }
                    Ty::String => {
                        *(this.add(field.offset) as *mut String) = String::read_byte(stream)?
                    }
                    Ty::Float => *(this.add(field.offset) as *mut f32) = f32::read_byte(stream)?,
                }
            }

            Ok(ret)
        }
    }
}
