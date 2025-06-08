use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Row, StatefulWidget, Table, Widget},
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
            Mode::Message(selected) => render_message(self, *selected, inner_area, buf),
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
    let mut table_state = app.message_table_state().borrow_mut();
    let rows = app
        .messages()
        .unwrap()
        .iter()
        .map(|m| Row::new(vec![
            format!("{:4}", m.id()),
            m.from().into(),
            m.subject().into()
        ]))
        .collect::<Vec<Row>>();
    let widths = [
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(50),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new())
        .header(
            Row::new(vec!["ID", "From", "Subject"])
                .style(Style::new().bold()),
        )
        .row_highlight_style(Style::new().reversed());

    StatefulWidget::render(table, area, buf, &mut *table_state);
}

fn render_message(app: &App, selected: usize, area: Rect, buf: &mut Buffer) {
    let message = app.view_message(selected);
    Paragraph::new(message)
        .bg(Color::Black)
        .render(area, buf);
}

fn render_blank(area: Rect, buf: &mut Buffer) {
    let text = "";

    let paragraph = Paragraph::new(text)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();

    paragraph.render(area, buf);
}
