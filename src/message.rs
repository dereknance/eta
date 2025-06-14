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
    pub fn new(id: u64, from: String, to: String, subject: String, body: String) -> Self {
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
    fn get_messages(&self) -> Result<&Vec<Message>, io::Error>;
    fn get_message(&self, id: u64) -> Result<&Message, io::Error>;
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
                    String::from("Hello there"),
                ),
                Message::new(
                    2,
                    String::from("alice@alice.me"),
                    String::from("me@me.me"),
                    String::from("TPS Reports"),
                    String::from(
                        "So uhh...if you could just get those done\n\
                        yeahh....that'd be greaaaat.",
                    ),
                ),
                Message::new(
                    3,
                    String::from("derek@dcn.dev"),
                    String::from("me@me.me"),
                    String::from("Big message"),
                    String::from(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing \
                        elit. Sed do eiusmod tempor incididunt ut labore et \
                        dolore magna aliqua. Ut enim ad minim veniam, quis \
                        nostrud exercitation ullamco laboris nisi ut aliquip ex\
                         ea commodo consequat. Duis aute irure dolor in \
                         reprehenderit in voluptate velit esse cillum dolore eu\
                          fugiat nulla pariatur. Excepteur sint occaecat \
                          cupidatat non proident, sunt in culpa qui officia \
                          deserunt mollit anim id est laborum.\n\
                        \n\
                        Sed ut perspiciatis unde omnis iste natus error sit \
                        voluptatem accusantium doloremque laudantium, totam rem\
                         aperiam, eaque ipsa quae ab illo inventore veritatis \
                         et quasi architecto beatae vitae dicta sunt explicabo.\
                          Nemo enim ipsam voluptatem quia voluptas sit \
                          aspernatur aut odit aut fugit, sed quia consequuntur\
                           magni dolores eos qui ratione voluptatem sequi \
                           nesciunt. Neque porro quisquam est, qui dolorem \
                           ipsum quia dolor sit amet, consectetur, adipisci \
                           velit.\n\
                        ",
                    ),
                ),
                Message::new(
                    4,
                    String::from("bob@bob.me"),
                    String::from("me@me.me"),
                    String::from("Hi"),
                    String::from("Hello there"),
                ),
                Message::new(
                    5,
                    String::from("alice@alice.me"),
                    String::from("me@me.me"),
                    String::from("TPS Reports"),
                    String::from(
                        "So uhh...if you could just get those done\n\
                        yeahh....that'd be greaaaat.",
                    ),
                ),
                Message::new(
                    6,
                    String::from("derek@dcn.dev"),
                    String::from("me@me.me"),
                    String::from("Big message"),
                    String::from(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing \
                        elit. Sed do eiusmod tempor incididunt ut labore et \
                        dolore magna aliqua. Ut enim ad minim veniam, quis \
                        nostrud exercitation ullamco laboris nisi ut aliquip ex\
                         ea commodo consequat. Duis aute irure dolor in \
                         reprehenderit in voluptate velit esse cillum dolore eu\
                          fugiat nulla pariatur. Excepteur sint occaecat \
                          cupidatat non proident, sunt in culpa qui officia \
                          deserunt mollit anim id est laborum.\n\
                        \n\
                        Sed ut perspiciatis unde omnis iste natus error sit \
                        voluptatem accusantium doloremque laudantium, totam rem\
                         aperiam, eaque ipsa quae ab illo inventore veritatis \
                         et quasi architecto beatae vitae dicta sunt explicabo.\
                          Nemo enim ipsam voluptatem quia voluptas sit \
                          aspernatur aut odit aut fugit, sed quia consequuntur\
                           magni dolores eos qui ratione voluptatem sequi \
                           nesciunt. Neque porro quisquam est, qui dolorem \
                           ipsum quia dolor sit amet, consectetur, adipisci \
                           velit.\n\
                        ",
                    ),
                ),
                Message::new(
                    7,
                    String::from("bob@bob.me"),
                    String::from("me@me.me"),
                    String::from("Hi"),
                    String::from("Hello there"),
                ),
                Message::new(
                    8,
                    String::from("alice@alice.me"),
                    String::from("me@me.me"),
                    String::from("TPS Reports"),
                    String::from(
                        "So uhh...if you could just get those done\n\
                        yeahh....that'd be greaaaat.",
                    ),
                ),
                Message::new(
                    9,
                    String::from("derek@dcn.dev"),
                    String::from("me@me.me"),
                    String::from("Big message"),
                    String::from(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing \
                        elit. Sed do eiusmod tempor incididunt ut labore et \
                        dolore magna aliqua. Ut enim ad minim veniam, quis \
                        nostrud exercitation ullamco laboris nisi ut aliquip ex\
                         ea commodo consequat. Duis aute irure dolor in \
                         reprehenderit in voluptate velit esse cillum dolore eu\
                          fugiat nulla pariatur. Excepteur sint occaecat \
                          cupidatat non proident, sunt in culpa qui officia \
                          deserunt mollit anim id est laborum.\n\
                        \n\
                        Sed ut perspiciatis unde omnis iste natus error sit \
                        voluptatem accusantium doloremque laudantium, totam rem\
                         aperiam, eaque ipsa quae ab illo inventore veritatis \
                         et quasi architecto beatae vitae dicta sunt explicabo.\
                          Nemo enim ipsam voluptatem quia voluptas sit \
                          aspernatur aut odit aut fugit, sed quia consequuntur\
                           magni dolores eos qui ratione voluptatem sequi \
                           nesciunt. Neque porro quisquam est, qui dolorem \
                           ipsum quia dolor sit amet, consectetur, adipisci \
                           velit.\n\
                        ",
                    ),
                ),
                Message::new(
                    10,
                    String::from("bob@bob.me"),
                    String::from("me@me.me"),
                    String::from("Hi"),
                    String::from("Hello there"),
                ),
                Message::new(
                    11,
                    String::from("alice@alice.me"),
                    String::from("me@me.me"),
                    String::from("TPS Reports"),
                    String::from(
                        "So uhh...if you could just get those done\n\
                        yeahh....that'd be greaaaat.",
                    ),
                ),
                Message::new(
                    12,
                    String::from("derek@dcn.dev"),
                    String::from("me@me.me"),
                    String::from("Big message"),
                    String::from(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing \
                        elit. Sed do eiusmod tempor incididunt ut labore et \
                        dolore magna aliqua. Ut enim ad minim veniam, quis \
                        nostrud exercitation ullamco laboris nisi ut aliquip ex\
                         ea commodo consequat. Duis aute irure dolor in \
                         reprehenderit in voluptate velit esse cillum dolore eu\
                          fugiat nulla pariatur. Excepteur sint occaecat \
                          cupidatat non proident, sunt in culpa qui officia \
                          deserunt mollit anim id est laborum.\n\
                        \n\
                        Sed ut perspiciatis unde omnis iste natus error sit \
                        voluptatem accusantium doloremque laudantium, totam rem\
                         aperiam, eaque ipsa quae ab illo inventore veritatis \
                         et quasi architecto beatae vitae dicta sunt explicabo.\
                          Nemo enim ipsam voluptatem quia voluptas sit \
                          aspernatur aut odit aut fugit, sed quia consequuntur\
                           magni dolores eos qui ratione voluptatem sequi \
                           nesciunt. Neque porro quisquam est, qui dolorem \
                           ipsum quia dolor sit amet, consectetur, adipisci \
                           velit.\n\
                        ",
                    ),
                ),
            ],
        }
    }
}

impl MessageProvider for DefaultMessageProvider {
    fn get_messages(&self) -> Result<&Vec<Message>, io::Error> {
        Ok(&self.messages)
    }

    fn get_message(&self, id: u64) -> Result<&Message, io::Error> {
        Ok(&self.messages[id as usize])
    }

    fn delete(&mut self, id: u64) -> Result<(), io::Error> {
        self.messages.retain(|m| m.id != id);
        Ok(())
    }

    fn len(&self) -> usize {
        self.messages.len()
    }
}
