use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Paragraph, Row, StatefulWidget, Table, Widget, Wrap},
};

use crate::app::{App, ComposeFocus, Mode};

impl Widget for &App {
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
    let message = app.get_message(selected);
    Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .render(area, buf);
}

fn render_compose(app: &App, _: &ComposeFocus, area: Rect, buf: &mut Buffer) {
    let message = format!("{:?}", app.mode());
    Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .render(area, buf);
}
