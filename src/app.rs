use std::{cell::RefCell, io};

use crate::{
    event::{AppEvent, Event, EventHandler},
    message::{DefaultMessageProvider, Message, MessageProvider},
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
    /// Event handler.
    events: EventHandler,
    /// Current application mode.
    mode: Mode,
    /// Message provider.
    messages: DefaultMessageProvider,
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

#[derive(Debug)]
pub enum Mode {
    MessageTable(MessageTableMode),
    Message(usize),
    Compose(ComposeFocus),
}

#[derive(Debug)]
pub enum MessageTableMode {
    Normal,
    MessageSent(MessageSentStatus),
}

#[derive(Debug)]
pub enum MessageSentStatus {
    Success,
    Failed(String),
}

#[derive(Debug)]
pub enum ComposeFocus {
    To(ComposeMode),
    Subject(ComposeMode),
    Message(ComposeMode),
}

#[derive(Debug)]
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
            events: EventHandler::new(),
            mode: Mode::MessageTable(MessageTableMode::Normal),
            messages: DefaultMessageProvider::new(event_sender),
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
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::SendMessage => self.send_message(),
                    AppEvent::Quit => self.quit(),
                    _ => {}
                },
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

        match &self.mode {
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
                }
                KeyCode::Right | KeyCode::Char('l') => {
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

    fn send_message(&mut self) {
        // TODO async transmit to an SMTP server -- read connection details from config

        // Reset state of compose fields
        self.compose_to_input = RefCell::new(TextArea::default());
        self.compose_subject_input = RefCell::new(TextArea::default());
        self.compose_message_input = RefCell::new(TextArea::default());

        // return to message table
        self.mode = Mode::MessageTable(MessageTableMode::MessageSent(MessageSentStatus::Success));
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn view_message(&mut self) {
        self.message_textarea = RefCell::new(TextArea::default());

        match self.message_table_state.borrow().selected() {
            Some(id) => {
                if (id + 1) as u64 != self.current_message.id() {
                    self.current_message = Box::new(
                        self.messages
                            .get_message(id as u64)
                            .expect("failed to get message")
                            .clone(),
                    )
                }
                self.mode = Mode::Message(id)
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
        self.mode = Mode::MessageTable(MessageTableMode::Normal)
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

    pub fn get_message(&self, selected: usize) -> String {
        let mut message = String::from("Not found");

        // TODO not compatible with async
        // for i in 0..self.loaded_messages.len() {
        //     if i == selected {
        //         message = self.messages().unwrap().get(i).unwrap().body().into();
        //     }
        // }

        message
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
}
