#[derive(Clone)]
pub struct Message {
    pub sender_id: usize,
    pub receiver_id: usize,
    pub content: String,
}

impl Message {
    pub fn new(sender_id: usize, receiver_id: usize, content: String) -> Self {
        Message {
            sender_id,
            receiver_id,
            content,
        }
    }
}
