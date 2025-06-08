use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::{App, Mode};

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" eta ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .fg(Color::Cyan)
            .bg(Color::Black);
        let inner_area = block.inner(area);

        block.render(area, buf);

        match self.mode() {
            Mode::Index => render_index(self, inner_area, buf),
            Mode::MessageTable => render_message_table(self, inner_area, buf),
            Mode::Blank => render_blank(inner_area, buf),
        };
    }
}

fn render_index(app: &App, area: Rect, buf: &mut Buffer) {
    let text = format!(
        "This is a tui template.\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press left and right to increment and decrement the counter respectively.\n\
            Counter: {}",
        app.counter()
    );

    let paragraph = Paragraph::new(text)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();

    paragraph.render(area, buf);
}

fn render_message_table(app: &App, area: Rect, buf: &mut Buffer) {
    let text = format!(
        "This is where the message table would go.\n\
            Press `Esc`, or `q` to return to the index, \n\
            or `Ctrl-C` to stop running."
    );

    let paragraph = Paragraph::new(text)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();

    paragraph.render(area, buf);
}

fn render_blank(area: Rect, buf: &mut Buffer) {
    let text = "";

    let paragraph = Paragraph::new(text)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();

    paragraph.render(area, buf);
}
