use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::{
    GameObject,
    io::bytes::{ReadStream, Readable, Writable},
    linking_context::LinkingContext,
    reflect::Reflect,
};

use super::io::{GameIoError, InputMemoryStream, OutputMemoryStream};

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PacketType {
    Hello,
    ReplicationData,
    Disconnect,
}

pub struct ObjectRegistry {
    fabrics: HashMap<u32, Box<dyn Fn() -> Arc<dyn GameObject>>>,
}

impl ObjectRegistry {
    pub fn register<T: Reflect + GameObject + 'static>(&mut self) -> &mut Self {
        self.fabrics
            .insert(T::type_id(), Box::new(|| Arc::new(T::create_instance())));
        self
    }

    pub fn create_game_object(&self, type_id: u32) -> Arc<dyn GameObject> {
        self.fabrics.get(&type_id).unwrap()()
    }
}

pub struct ReplicationManager {
    objects_to_me: HashSet<usize>,
}

impl ReplicationManager {
    pub fn new() -> Self {
        Self {
            objects_to_me: Default::default(),
        }
    }

    fn replicate_into_stream(
        &self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
        go: &Arc<dyn GameObject>,
    ) {
        go.write_byte(stream).unwrap();
        go.class_id().write_byte(stream).unwrap();

        todo!(
            /* Need add reflection methods to GameObject and write data from dyn data ptr */
            /* Ugly architecture */
            /* Need refactoring */
        )
    }

    pub fn replicate_world_state(
        &self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
        gos: &[Arc<dyn GameObject>],
    ) {
        (PacketType::ReplicationData as u8)
            .write_byte(stream)
            .unwrap();
        for go in gos {
            self.replicate_into_stream(stream, go);
        }
    }

    fn recv_replicated_object(
        &mut self,
        input: &mut InputMemoryStream<'_, '_, LinkingContext>,
        registry: &ObjectRegistry,
    ) -> usize {
        let go = Arc::<dyn GameObject>::read_byte(input);
        let class_id = u32::read_byte(input).unwrap();

        match go {
            Ok(go) => go.id(),
            Err(super::io::GameIoError::UnregisteredGameObject(id)) => {
                let go = registry.create_game_object(class_id);
                input.ctx.insert_game_object(go.clone(), id);

                go.id()
            }
            _ => panic!(),
        }
    }

    pub fn recv_replicated_objects(
        &mut self,
        input: &mut InputMemoryStream<'_, '_, LinkingContext>,
        registry: &ObjectRegistry,
    ) {
        let mut set = HashSet::new();

        while input.remaining_bit_count() > 0 {
            let go = self.recv_replicated_object(input, registry);
            set.insert(go);
        }

        for go in &self.objects_to_me {
            if !set.contains(go) {
                input.ctx.remove_game_object(*go);
            }
        }

        self.objects_to_me = set;
    }

    pub fn replicate_create(
        &mut self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
        go: &Arc<dyn GameObject>,
    ) {
        let header = ReplicationHeader {
            action: ReplicationAction::Create,
            network_id: stream.ctx.get_network_id(go, false).unwrap(),
            class_id: go.class_id(),
        };
        header.write_byte(stream).unwrap();
        /* todo() */
    }

    pub fn replicate_update(
        &mut self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
        go: &Arc<dyn GameObject>,
    ) {
        let header = ReplicationHeader {
            action: ReplicationAction::Update,
            network_id: stream.ctx.get_network_id(go, false).unwrap(),
            class_id: go.class_id(),
        };
        header.write_byte(stream).unwrap();
        /* todo() */
    }

    pub fn replicate_destroy(
        &mut self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
        go: &Arc<dyn GameObject>,
    ) {
        let header = ReplicationHeader {
            action: ReplicationAction::Destroy,
            network_id: stream.ctx.get_network_id(go, false).unwrap(),
            class_id: go.class_id(),
        };
        header.write_byte(stream).unwrap();
    }

    fn process_replication_action(
        &mut self,
        stream: &mut InputMemoryStream<'_, '_, LinkingContext>,
    ) {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ReplicationAction {
    Create,
    Update,
    Destroy,
}

pub struct ReplicationHeader {
    pub action: ReplicationAction,
    pub network_id: usize,
    pub class_id: u32,
}

impl Writable<OutputMemoryStream<'_, '_, LinkingContext>> for ReplicationHeader {
    fn write_byte(
        &self,
        stream: &mut OutputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<(), GameIoError> {
        (self.action as u8).write_byte(stream)?;
        self.network_id.write_byte(stream)?;
        self.class_id.write_byte(stream)?;

        Ok(())
    }
}

impl Readable<InputMemoryStream<'_, '_, LinkingContext>> for ReplicationHeader {
    fn read_byte(
        stream: &mut InputMemoryStream<'_, '_, LinkingContext>,
    ) -> Result<Self, GameIoError> {
        Ok(Self {
            action: unsafe { std::mem::transmute(stream.read_u8()?) },
            network_id: stream.read_usize()?,
            class_id: stream.read_u32()?,
        })
    }
}
