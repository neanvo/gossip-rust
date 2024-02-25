use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub struct Message {
    pub content: String,
    pub sender: String,
}

impl Message {

    pub fn new(content: String, sender: String) -> Self {
        Self { content, sender }
    }

    pub fn new_random(len: usize, sender: String) -> Self {
        let rng = thread_rng();
        let rand_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        Self::new(rand_string, sender)
    }
}

