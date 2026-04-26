use std::any::Any;
use std::sync::{Arc, LazyLock, Mutex};

pub struct MPMCMessage {
    pub id: u64,
    pub topic: &'static str,
    pub body: Box<dyn Any + Send>,
}

pub struct TypedMPMCMessage<'a, T: Any> {
    pub id: u64,
    pub topic: &'static str,
    pub body: &'a T,
}

impl MPMCMessage {
    pub fn as_ref<T: Any>(&self) -> &T {
        self.body.downcast_ref::<T>().unwrap()
    }

    pub fn as_typed<T: Any>(&self) -> TypedMPMCMessage<T> {
        TypedMPMCMessage {
            id: self.id,
            topic: self.topic,
            body: self.body.downcast_ref::<T>().unwrap(),
        }
    }
}

pub struct MPMCBus {
    id_counter: u64,
    pub message_log: Arc<Mutex<Vec<MPMCMessage>>>,
}

impl MPMCBus {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            message_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push<T: Any + Send>(&mut self, topic: &'static str, body: T) {
        self.message_log.lock().unwrap().push(MPMCMessage {
            id: self.id_counter + 1,
            topic,
            body: Box::new(body),
        });
        self.id_counter += 1;
    }

    pub fn read_topic_callback<T: Any>(
        &self,
        topic: &'static str,
        from: u64,
        callback: impl Fn(TypedMPMCMessage<T>),
    ) {
        self.message_log
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.id >= from && x.topic.eq(topic))
            .for_each(|x| {
                callback(x.as_typed());
            });
    }

    pub fn read_topic_callback_mut<T: Any>(
        &self,
        topic: &'static str,
        from: u64,
        mut callback: impl FnMut(TypedMPMCMessage<T>),
    ) {
        self.message_log
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.id >= from && x.topic.eq(topic))
            .for_each(|x| {
                callback(x.as_typed());
            });
    }
}

pub static MPMC_BUS: LazyLock<MPMCBus> = LazyLock::new(|| MPMCBus::new());
