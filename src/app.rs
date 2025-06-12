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
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    MessageTable,
    Message(usize),
    Compose(ComposeFocus),
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
        let mut app = Self {
            running: true,
            events: EventHandler::new(),
            mode: Mode::MessageTable,
            messages: DefaultMessageProvider::new(),
            message_table_state: RefCell::new(TableState::default().with_selected(0)),
            message_scroll_state: ScrollbarState::default(),
            compose_message_input: RefCell::new(TextArea::default()),
            compose_to_input: RefCell::new(TextArea::default()),
            compose_subject_input: RefCell::new(TextArea::default()),
        };
        app.message_scroll_state = ScrollbarState::new(app.messages.len() - 1);

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
            Mode::MessageTable => match key_event.code {
                KeyCode::Enter => self.view_message(),
                KeyCode::Char('c') => self.compose_message(),
                KeyCode::Char('j') | KeyCode::Down => self.next_message(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_message(),
                KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                _ => {}
            },
            Mode::Message(_) => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.mode = Mode::MessageTable,
                // TODO scroll
                _ => {}
            },
            Mode::Compose(focus) => match focus {
                ComposeFocus::To(compose_mode) => match compose_mode {
                    ComposeMode::Normal => match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') => self.mode = Mode::MessageTable,
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
                        KeyCode::Esc | KeyCode::Char('q') => self.mode = Mode::MessageTable,
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
                        KeyCode::Esc | KeyCode::Char('q') => self.mode = Mode::MessageTable,
                        KeyCode::Char('S') => {
                            // send an app event to send the message!
                            self.events.send(AppEvent::SendMessage);
                        }
                        KeyCode::Enter => {
                            self.mode = Mode::Compose(ComposeFocus::Message(ComposeMode::Editing))
                        }
                        KeyCode::Tab => {
                            self.mode = Mode::Compose(ComposeFocus::To(ComposeMode::Normal))
                        }
                        KeyCode::Up => {
                            self.compose_message_input.get_mut().scroll(Scrolling::HalfPageUp);
                        }
                        KeyCode::Down => {
                            self.compose_message_input.get_mut().scroll(Scrolling::HalfPageDown);
                        }
                        _ => {}
                    },
                    ComposeMode::Editing => match key_event.code {
                        KeyCode::Esc => {
                            self.mode = Mode::Compose(ComposeFocus::Message(ComposeMode::Normal))
                        }
                        KeyCode::Up => {
                            self.compose_message_input.get_mut().move_cursor(CursorMove::Up);
                        }
                        KeyCode::Down => {
                            self.compose_message_input.get_mut().move_cursor(CursorMove::Down);
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
        // TODO show a message until keypress in status bar
        self.mode = Mode::MessageTable
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn view_message(&mut self) {
        match self.message_table_state.borrow().selected() {
            Some(id) => self.mode = Mode::Message(id),
            None => {}
        }
    }

    fn next_message(&mut self) {
        let mut state = self.message_table_state.borrow_mut();
        let i = match state.selected() {
            Some(i) => {
                if i >= self.messages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
        // self.message_scroll_state = self.message_scroll_state.position(i);
    }

    fn previous_message(&mut self) {
        let mut state = self.message_table_state.borrow_mut();
        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    self.messages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
        // self.message_scroll_state = self.message_scroll_state.position(i);
    }

    fn compose_message(&mut self) {
        self.mode = Mode::Compose(ComposeFocus::To(ComposeMode::Normal));
    }

    pub fn get_message(&self, selected: usize) -> String {
        let mut message = String::from("Not found");

        for i in 0..self.messages.len() {
            if i == selected {
                message = self.messages().unwrap().get(i).unwrap().body().into();
            }
        }

        message
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn messages(&self) -> Result<&Vec<Message>, io::Error> {
        self.messages.get()
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
}
