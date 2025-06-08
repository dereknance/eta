use std::{cell::RefCell, io};

use crate::{event::{AppEvent, Event, EventHandler}, message::{DefaultMessageProvider, Message, MessageProvider}};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers}, widgets::{ScrollbarState, TableState}, DefaultTerminal
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// Counter.
    counter: u8,
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
}

#[derive(Debug)]
pub enum Mode {
    Index,
    MessageTable,
    Blank,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            mode: Mode::Index,
            messages: DefaultMessageProvider::new(),
            message_table_state: RefCell::new(TableState::default().with_selected(0)),
            message_scroll_state: ScrollbarState::default(),
        };
        app.message_scroll_state = ScrollbarState::new(app.messages.len() - 1);
        app
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame|
                frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) =>
                        self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            // Ctrl-C is the escape hatch to close the program
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // Mode-dependent keypresses
            KeyCode::Esc => self.handle_esc_key(),
            KeyCode::Char('q') => self.handle_q_key(),
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Char('b') => self.handle_b_key(),
            KeyCode::Char('m') => self.handle_m_key(),
            KeyCode::Char('j') => self.handle_j_key(),
            KeyCode::Char('k') => self.handle_k_key(),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    fn tick(&self) {}

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn handle_b_key(&mut self) {
        match self.mode {
            Mode::Index => self.mode = Mode::Blank,
            _ => ()
        };
    }

    fn handle_m_key(&mut self) {
        match self.mode {
            // TODO this _may_ need to wind up asynchronously loading
            // messages
            Mode::Index => self.mode = Mode::MessageTable,
            _ => ()
        }
    }

    fn handle_esc_key(&mut self) {
        match self.mode {
            Mode::Index => self.events.send(AppEvent::Quit),
            _ => self.mode = Mode::Index,
        };
    }

    fn handle_q_key(&mut self) {
        match self.mode {
            Mode::Index => self.events.send(AppEvent::Quit),
            _ => self.mode = Mode::Index,
        };
    }

    fn handle_j_key(&mut self) {
        match self.mode {
            Mode::MessageTable => self.next_message(),
            _ => ()
        }
    }

    fn handle_k_key(&mut self) {
        match self.mode {
            Mode::MessageTable => self.previous_message(),
            _ => ()
        }
    }

    fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
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

    pub fn counter(&self) -> u8 {
        self.counter
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
}
