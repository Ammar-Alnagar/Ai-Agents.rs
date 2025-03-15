#[derive(Clone)]
pub struct Message {
    pub sender: usize,
    pub receiver: usize,
    pub content: String,
}

impl Message {
    pub fn new(sender: usize, receiver: usize, content: String) -> Self {
        Message {
            sender,
            receiver,
            content,
        }
    }
}
