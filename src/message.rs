use std::{str::FromStr, sync::Arc};

use tokio::sync::mpsc;

use crate::event::{AppEvent, Event};

use futures::TryStreamExt;
use sqlx::{Row, sqlite::SqliteRow};

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
    fn get_messages(&self);
    fn get_message(&self, id: u64);
    fn send_message(&self, message: &Message);
}

#[derive(Debug)]
pub struct DefaultMessageProvider {
    messages: Vec<Message>,
    event_sender: mpsc::UnboundedSender<Event>,
}

#[derive(Debug)]
pub struct SqliteMessageProvider {
    connection: Arc<sqlx::SqlitePool>,
    event_sender: mpsc::UnboundedSender<Event>,
}

impl DefaultMessageProvider {
    #[allow(dead_code)]
    pub fn new(event_sender: mpsc::UnboundedSender<Event>) -> Self {
        DefaultMessageProvider {
            event_sender,
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

    #[allow(dead_code)]
    pub fn init(&mut self) -> color_eyre::Result<()> {
        Ok(())
    }
}

impl MessageProvider for DefaultMessageProvider {
    fn get_messages(&self) {
        let event_sender = self.event_sender.clone();
        let messages = self.messages.clone();
        tokio::spawn(async move {
            // bake in some delay
            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

            let app_event = AppEvent::MessagesLoaded(messages);
            let event = Event::App(app_event);
            let _ = event_sender.send(event);
        });
    }

    fn get_message(&self, id: u64) {
        let event_sender = self.event_sender.clone();
        // count from zero since I'm using a vector for these "static" messages.
        let vector_index = (id as usize).saturating_sub(1);
        let message_body = String::from(self.messages[vector_index].body());

        tokio::spawn(async move {
            // bake in some delay
            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

            let app_event = AppEvent::MessageBodyLoaded(id, message_body);
            let event = Event::App(app_event);
            let _ = event_sender.send(event);
        });
    }

    fn send_message(&self, message: &Message) {
        let event_sender = self.event_sender.clone();
        let message = message.clone();

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

            let app_event =
                AppEvent::MessageSent(Some(format!("failed sending to {}", message.to())));
            let event = Event::App(app_event);
            let _ = event_sender.send(event);
        });
    }
}

impl SqliteMessageProvider {
    pub fn new(event_sender: mpsc::UnboundedSender<Event>) -> color_eyre::Result<Self> {
        let opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite://messages.db")?
            .create_if_missing(true);
        let connection = Arc::new(sqlx::SqlitePool::connect_lazy_with(opts));

        let provider = Self {
            connection,
            event_sender,
        };

        Ok(provider)
    }

    /// Create the necessary schema if it does not already exist.
    pub async fn init(&self) -> color_eyre::Result<()> {
        // deref to get the protected type, then make a reference
        // init and seed are called once, and in order, so I'm not worried about
        // concurrency
        let conn = &*self.connection;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_addr TEXT NOT NULL,
                to_addr TEXT NOT NULL,
                subject TEXT NOT NULL,
                body TEXT NOT NULL
            )",
        )
        .execute(conn)
        .await?;

        let result: (u64,) = sqlx::query_as("SELECT COUNT(id) FROM messages")
            .fetch_one(conn)
            .await?;
        let count = result.0;

        if count != 0 {
            return Ok(());
        }

        self.seed_messages().await?;

        Ok(())
    }

    async fn seed_messages(&self) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(
            "INSERT INTO messages (id, from_addr, to_addr, subject, body) VALUES (
                1, 'alice@example.com', 'bob@example.com', 'Hello there', 'Bob,\n\nI hope you are well.\n\nRegards,\nAlice\n'
        )",
        )
        .execute(&*self.connection)
        .await?;

        Ok(())
    }
}

impl MessageProvider for SqliteMessageProvider {
    fn get_messages(&self) {
        let event_sender = self.event_sender.clone();
        let connection = self.connection.clone();

        tokio::spawn(async move {
            let mut messages = vec![];

            let mut stream =
                sqlx::query("SELECT id, from_addr, to_addr, subject FROM messages ORDER BY id")
                    .map(|row: SqliteRow| Message {
                        id: row.get(0),
                        from: row.get(1),
                        to: row.get(2),
                        subject: row.get(3),
                        body: String::from(""),
                    })
                    .fetch(&*connection);

            loop {
                let result = stream.try_next().await;
                if let Err(e) = result {
                    let app_event = AppEvent::Error(e.to_string());
                    let event = Event::App(app_event);
                    let _ = event_sender.send(event);
                    return;
                }

                if let Some(message) = result.unwrap() {
                    messages.push(message);
                } else {
                    break;
                }
            }

            let app_event = AppEvent::MessagesLoaded(messages);
            let event = Event::App(app_event);
            let _ = event_sender.send(event);
        });
    }

    fn get_message(&self, id: u64) {
        let event_sender = self.event_sender.clone();
        let connection = self.connection.clone();

        tokio::spawn(async move {
            let result = sqlx::query("SELECT body FROM messages WHERE id = ?")
                .bind(id as i64)
                .fetch_one(&*connection)
                .await;

            let app_event = match result {
                Ok(row) => AppEvent::MessageBodyLoaded(id, row.get("body")),
                Err(e) => AppEvent::Error(e.to_string()),
            };

            let event = Event::App(app_event);
            let _ = event_sender.send(event);
        });
    }

    fn send_message(&self, message: &Message) {
        todo!()
    }
}
