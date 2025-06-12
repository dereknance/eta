use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Row, StatefulWidget, Table, Widget, Wrap},
};

use crate::app::{App, ComposeFocus, ComposeMode, Mode};

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode() {
            Mode::MessageTable => render_message_table(self, area, buf),
            Mode::Message(selected) => render_message(self, *selected, area, buf),
            Mode::Compose(focus) => render_compose(self, focus, area, buf),
        };
    }
}

fn render_message_table(app: &App, area: Rect, buf: &mut Buffer) {
    let mut table_state = app.message_table_state().borrow_mut();
    let rows = app
        .messages()
        .unwrap()
        .iter()
        .map(|m| {
            Row::new(vec![
                format!("{:4}", m.id()),
                m.from().into(),
                m.subject().into(),
            ])
        })
        .collect::<Vec<Row>>();
    let widths = [
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(50),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new())
        .header(Row::new(vec!["ID", "From", "Subject"]).style(Style::new().bold()))
        .row_highlight_style(Style::new().reversed());

    StatefulWidget::render(table, area, buf, &mut *table_state);
}

fn render_message(app: &App, selected: usize, area: Rect, buf: &mut Buffer) {
    let message = app.get_message(selected);
    Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .render(area, buf);
}

fn render_compose(app: &App, focus: &ComposeFocus, area: Rect, buf: &mut Buffer) {
    let [to_area, subject_area, message_area, keybind_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);
    let paragraph = Paragraph::new(format!("{:?}", app.mode()));
    let mut message = app.compose_message_input().borrow_mut();
    let subject = Paragraph::new("Subject . . .");
    let keybinds = match focus {
        ComposeFocus::Message(ComposeMode::Editing) => Line::from("Esc: Stop editing"),
        _ => Line::from("q: Back Tab: Next field Enter: Select field"),
    };

    let default_style = Style::default();
    let reversed_style = default_style.reversed();
    message.set_cursor_line_style(default_style);
    message.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Message ")
            .title_style(match focus {
                ComposeFocus::Message(ComposeMode::Normal) => reversed_style,
                _ => default_style,
            }),
    );
    message.set_cursor_style(match focus {
        ComposeFocus::Message(ComposeMode::Editing) => reversed_style,
        _ => default_style, // hide cursor
    });

    paragraph.render(to_area, buf);
    subject.render(subject_area, buf);
    message.render(message_area, buf);
    keybinds.render(keybind_area, buf);
}
