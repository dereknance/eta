use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
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
}

#[derive(Debug)]
pub enum Mode {
    Index,
    Blank,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            mode: Mode::Index,
        }
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

    fn handle_esc_key(&mut self) {
        match self.mode {
            Mode::Index => self.events.send(AppEvent::Quit),
            Mode::Blank => self.mode = Mode::Index,
        };
    }

    fn handle_q_key(&mut self) {
        match self.mode {
            Mode::Index => self.events.send(AppEvent::Quit),
            Mode::Blank => self.mode = Mode::Index,
        };
    }

    fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }
}
