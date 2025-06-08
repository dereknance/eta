use std::io;

#[derive(Debug, Default, Clone)]
pub struct Message {
    id: u64,
    from: String,
    to: String,
    subject: String,
    body: String,
}

impl Message {
    pub fn new(
        id: u64,
        from: String,
        to: String,
        subject: String,
        body: String
    ) -> Self
    {
        Message {
            id,
            from,
            to,
            subject,
            body,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_from(&mut self, from: String) {
        self.from = from;
    }

    pub fn set_to(&mut self, to: String) {
        self.to = to;
    }

    pub fn set_subject(&mut self, subject: String) {
        self.subject = subject;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

pub trait MessageProvider {
    fn get(&self) -> Result<&Vec<Message>, io::Error>;
    #[allow(dead_code)]
    fn delete(&mut self, id: u64) -> Result<(), io::Error>;
    fn len(&self) -> usize;
}

#[derive(Debug)]
pub struct DefaultMessageProvider {
    messages: Vec<Message>,
}

impl DefaultMessageProvider {
    pub fn new() -> Self {
        DefaultMessageProvider {
            messages: vec![
                Message::new(
                    1,
                    String::from("bob@bob.me"),
                    String::from("me@me.me"),
                    String::from("Hi"),
                    String::from("Hello there")),
                Message::new(
                    2,
                    String::from("alice@alice.me"),
                    String::from("me@me.me"),
                    String::from("TPS Reports"),
                    String::from("Hello there")),
            ]
        }
    }
}

impl MessageProvider for DefaultMessageProvider {
    fn get(&self) -> Result<&Vec<Message>, io::Error> {
        Ok(&self.messages)
    }

    fn delete(&mut self, id: u64) -> Result<(), io::Error> {
        self.messages.retain(|m| m.id != id);
        Ok(())
    }

    fn len(&self) -> usize {
        self.messages.len()
    }
}