use std::{cell::RefCell};

use crate::{
    event::{AppEvent, Event, EventHandler},
    message::{DefaultMessageProvider, Message, MessageProvider, SqliteMessageProvider},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::{ScrollbarState, TableState},
};
use tui_textarea::{CursorMove, Scrolling, TextArea};

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    running: bool,
    /// Only render when necessary -- save those sweet CPU cycles.
    needs_render: bool,
    /// Event handler.
    events: EventHandler,
    /// Current application mode.
    mode: Mode,
    /// Message provider.
    messages: SqliteMessageProvider,
    /// Message table state.
    message_table_state: RefCell<TableState>,
    /// Message table scrollbar state.
    message_scroll_state: ScrollbarState,
    compose_message_input: RefCell<TextArea<'a>>,
    compose_to_input: RefCell<TextArea<'a>>,
    compose_subject_input: RefCell<TextArea<'a>>,
    message_textarea: RefCell<TextArea<'a>>,
    current_message: Box<Message>,
    loaded_messages: Box<Vec<Message>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    LoadingMessages,
    MessageTable(MessageTableMode),
    Message(usize),
    Compose(ComposeFocus),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageTableMode {
    Normal,
    MessageSent(MessageSentStatus),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageSentStatus {
    Success,
    Failed(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComposeFocus {
    To(ComposeMode),
    Subject(ComposeMode),
    Message(ComposeMode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComposeMode {
    Normal,
    Editing,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let event_handler = EventHandler::new();
        let event_sender = event_handler.sender();

        let mut app = Self {
            running: true,
            needs_render: true,
            events: event_handler,
            mode: Mode::MessageTable(MessageTableMode::Normal),
            messages: SqliteMessageProvider::new(event_sender).unwrap(),
            message_table_state: RefCell::new(TableState::default().with_selected(0)),
            message_scroll_state: ScrollbarState::default(),
            compose_message_input: RefCell::new(TextArea::default()),
            compose_to_input: RefCell::new(TextArea::default()),
            compose_subject_input: RefCell::new(TextArea::default()),
            message_textarea: RefCell::new(TextArea::default()),
            current_message: Box::new(Message::default()),
            loaded_messages: Box::new(vec![]),
        };
        app.message_scroll_state = ScrollbarState::new(app.loaded_messages.len().saturating_sub(1));

        app
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        // allow the message provider to initialize
        self.messages.init().await?;

        // start by loading messages, since we start on the message table
        self.messages.get_messages();

        while self.running {
            if self.needs_render {
                terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            }
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => {
                    self.needs_render = true;
                    match app_event {
                        AppEvent::MessagesLoaded(messages) => self.set_loaded_messages(messages),
                        AppEvent::MessageBodyLoaded(id, body) => self.set_current_message(id, body),
                        AppEvent::MessageSent(option) => self.set_message_sent_status(option),
                        AppEvent::SendMessage => self.send_message(),
                        AppEvent::Quit => self.quit(),
                        AppEvent::Error(e) => self.show_error(e)?,
                    };
                }
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        // escape hatch
        if key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('c') {
            self.events.send(AppEvent::Quit);
            return Ok(());
        }

        self.needs_render = true;

        match &self.mode {
            Mode::LoadingMessages => {}
            Mode::MessageTable(_) => match key_event.code {
                KeyCode::Enter => self.view_message(),
                KeyCode::Char('c') => self.compose_message(),
                KeyCode::Char('j') | KeyCode::Down => self.next_message(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_message(),
                KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                _ => {}
            },
            Mode::Message(_) => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.mode = Mode::MessageTable(MessageTableMode::Normal)
                }
                KeyCode::Up | KeyCode::PageUp | KeyCode::Char('k') => {
                    self.message_textarea
                        .get_mut()
                        .scroll(Scrolling::HalfPageUp);
                }
                KeyCode::Down | KeyCode::PageDown | KeyCode::Char('j') => {
                    self.message_textarea
                        .get_mut()
                        .scroll(Scrolling::HalfPageDown);
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.message_textarea.get_mut().scroll((0, -40));
                    self.message_textarea.get_mut().scroll((0, -40));
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.message_textarea.get_mut().scroll((0, 40));
                    self.message_textarea.get_mut().scroll((0, 40));
                }
                _ => {}
            },
            Mode::Compose(focus) => match focus {
                ComposeFocus::To(compose_mode) => match compose_mode {
                    ComposeMode::Normal => match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.mode = Mode::MessageTable(MessageTableMode::Normal)
                        }
                        KeyCode::Enter => {
                            self.mode = Mode::Compose(ComposeFocus::To(ComposeMode::Editing))
                        }
                        KeyCode::Tab => {
                            self.mode = Mode::Compose(ComposeFocus::Subject(ComposeMode::Normal))
                        }
                        _ => {}
                    },
                    ComposeMode::Editing => match key_event.code {
                        KeyCode::Esc => {
                            self.mode = Mode::Compose(ComposeFocus::To(ComposeMode::Normal))
                        }
                        KeyCode::Enter | KeyCode::Tab => {
                            self.mode = Mode::Compose(ComposeFocus::Subject(ComposeMode::Normal))
                        }
                        _ => {
                            self.compose_to_input
                                .get_mut()
                                .input_without_shortcuts(key_event);
                        }
                    },
                },
                ComposeFocus::Subject(compose_mode) => match compose_mode {
                    ComposeMode::Normal => match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.mode = Mode::MessageTable(MessageTableMode::Normal)
                        }
                        KeyCode::Enter => {
                            self.mode = Mode::Compose(ComposeFocus::Subject(ComposeMode::Editing))
                        }
                        KeyCode::Tab => {
                            self.mode = Mode::Compose(ComposeFocus::Message(ComposeMode::Normal))
                        }
                        _ => {}
                    },
                    ComposeMode::Editing => match key_event.code {
                        KeyCode::Esc => {
                            self.mode = Mode::Compose(ComposeFocus::Subject(ComposeMode::Normal))
                        }
                        KeyCode::Enter | KeyCode::Tab => {
                            self.mode = Mode::Compose(ComposeFocus::Message(ComposeMode::Normal))
                        }
                        _ => {
                            self.compose_subject_input
                                .get_mut()
                                .input_without_shortcuts(key_event);
                        }
                    },
                },
                ComposeFocus::Message(compose_mode) => match compose_mode {
                    ComposeMode::Normal => match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.mode = Mode::MessageTable(MessageTableMode::Normal)
                        }
                        KeyCode::Char('S') => {
                            self.events.send(AppEvent::SendMessage);
                        }
                        KeyCode::Enter => {
                            self.mode = Mode::Compose(ComposeFocus::Message(ComposeMode::Editing))
                        }
                        KeyCode::Tab => {
                            self.mode = Mode::Compose(ComposeFocus::To(ComposeMode::Normal))
                        }
                        KeyCode::Up => {
                            self.compose_message_input
                                .get_mut()
                                .scroll(Scrolling::HalfPageUp);
                        }
                        KeyCode::Down => {
                            self.compose_message_input
                                .get_mut()
                                .scroll(Scrolling::HalfPageDown);
                        }
                        _ => {}
                    },
                    ComposeMode::Editing => match key_event.code {
                        KeyCode::Esc => {
                            self.mode = Mode::Compose(ComposeFocus::Message(ComposeMode::Normal))
                        }
                        KeyCode::Up => {
                            self.compose_message_input
                                .get_mut()
                                .move_cursor(CursorMove::Up);
                        }
                        KeyCode::Down => {
                            self.compose_message_input
                                .get_mut()
                                .move_cursor(CursorMove::Down);
                        }
                        _ => {
                            self.compose_message_input
                                .get_mut()
                                .input_without_shortcuts(key_event);
                        }
                    },
                },
            },
        }

        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    fn tick(&self) {}

    fn show_error(&self, error_message: String) -> color_eyre::Result<()> {
        Err(color_eyre::eyre::eyre!(error_message))
    }

    fn send_message(&mut self) {
        let mut message = Box::new(Message::default());
        // TODO set from using config
        message.set_to(self.compose_to_input.borrow().lines()[0].clone());
        message.set_subject(self.compose_subject_input.borrow().lines()[0].clone());
        message.set_body(self.compose_message_input.borrow().lines().join("\n"));
        // TODO async transmit to an SMTP server -- read connection details from config
        self.messages.send_message(&message);

        // Reset state of compose fields
        self.compose_to_input = RefCell::new(TextArea::default());
        self.compose_subject_input = RefCell::new(TextArea::default());
        self.compose_message_input = RefCell::new(TextArea::default());

        // return to message table
        // self.mode = Mode::MessageTable(MessageTableMode::MessageSent(MessageSentStatus::Success));
        self.mode = Mode::MessageTable(MessageTableMode::Normal);
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn view_message(&mut self) {
        match self.message_table_state.borrow().selected() {
            Some(id) => {
                self.message_textarea = RefCell::new(TextArea::default());
                let message_id = (id + 1) as u64;
                // ask the provider to load the message body
                self.messages.get_message(message_id);
                self.mode = Mode::Message(id);
                self.needs_render = true;
            }
            None => {}
        }
    }

    fn next_message(&mut self) {
        let mut state = self.message_table_state.borrow_mut();
        let i = match state.selected() {
            Some(i) => {
                if i >= self.loaded_messages.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));

        // clear any status messages
        self.mode = Mode::MessageTable(MessageTableMode::Normal);
        self.needs_render = true;
    }

    fn previous_message(&mut self) {
        let mut state = self.message_table_state.borrow_mut();
        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    self.loaded_messages.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));

        // clear any status messages
        self.mode = Mode::MessageTable(MessageTableMode::Normal)
    }

    fn compose_message(&mut self) {
        self.mode = Mode::Compose(ComposeFocus::To(ComposeMode::Normal));
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.loaded_messages
    }

    pub fn message_table_state(&self) -> &RefCell<TableState> {
        &self.message_table_state
    }

    pub fn compose_message_text(&self) -> String {
        self.compose_message_input.borrow().lines().join("\n")
    }

    pub fn compose_message_input(&self) -> &RefCell<TextArea<'a>> {
        &self.compose_message_input
    }

    pub fn compose_to_input(&self) -> &RefCell<TextArea<'a>> {
        &self.compose_to_input
    }

    pub fn compose_subject_input(&self) -> &RefCell<TextArea<'a>> {
        &self.compose_subject_input
    }

    pub fn message_textarea(&self) -> &RefCell<TextArea<'a>> {
        &self.message_textarea
    }

    pub fn current_message(&self) -> &Message {
        &self.current_message
    }

    fn set_loaded_messages(&mut self, messages: Vec<Message>) {
        self.loaded_messages = Box::new(messages);
        // also set the first row of the message table as selected if there is
        // not yet anything selected.
        let mut table_state = self.message_table_state.borrow_mut();
        if self.loaded_messages.len() > 0 && table_state.selected() == None {
            table_state.select(Some(0));
        }
    }

    fn set_current_message(&mut self, id: u64, body: String) {
        if id != self.current_message.id() {
            for message in self.loaded_messages.iter() {
                if id == message.id() {
                    self.current_message = Box::new(message.clone());
                    self.current_message.set_body(body);
                    break;
                }
            }
        }

        match self.message_table_state.borrow().selected() {
            Some(table_id) => {
                self.mode = Mode::Message(table_id);
                self.needs_render = true;
            }
            None => {}
        }

        let message = &self.current_message;
        self.message_textarea.get_mut().insert_str(format!(
            "From: {}\nTo: {}\nSubject: {}\n\n{}",
            message.from(),
            message.to(),
            message.subject(),
            message.body()
        ));
    }
    
    fn set_message_sent_status(&mut self, status: Option<String>) {
        let sent_status = match status {
            Some(str) => MessageSentStatus::Failed(str),
            None => MessageSentStatus::Success,
        };
        let table_mode = MessageTableMode::MessageSent(sent_status);
        let app_mode = Mode::MessageTable(table_mode);

        match self.mode {
            Mode::MessageTable(_) => self.mode = app_mode,
            _ => {}
        }
    }
}
