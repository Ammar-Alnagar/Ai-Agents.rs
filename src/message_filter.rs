// src/message_filter.rs
use crate::message::Message;

pub trait MessageFilter {
    fn allow(&self, message: &Message) -> bool;
}

pub struct TypeFilter {
    allowed_types: Vec<&'static str>,
}

impl TypeFilter {
    pub fn new(types: Vec<&'static str>) -> Self {
        TypeFilter { allowed_types: types }
    }
}

impl MessageFilter for TypeFilter {
    fn allow(&self, message: &Message) -> bool {
        self.allowed_types.iter().any(|&t| message.content.contains(t))
    }
}
