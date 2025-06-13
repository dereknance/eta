use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Row, StatefulWidget, Table, Widget, Wrap},
};

use crate::app::{App, ComposeFocus, ComposeMode, MessageTableMode, Mode};

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode() {
            Mode::MessageTable(mode) => render_message_table(self, mode, area, buf),
            Mode::Message(selected) => render_message(self, *selected, area, buf),
            Mode::Compose(focus) => render_compose(self, focus, area, buf),
        };
    }
}

fn render_message_table(app: &App, mode: &MessageTableMode, area: Rect, buf: &mut Buffer) {
    let keybinds_text = " j:Up  k:Down  Enter:View  c:Compose ";
    let keybinds_text_len = keybinds_text.len() as u16;
    let status_text = match mode {
        MessageTableMode::Normal => String::from(""),
        MessageTableMode::MessageSent(status) => match status {
            crate::app::MessageSentStatus::Success => String::from(" Message sent "),
            crate::app::MessageSentStatus::Failed(e) => format!(" Error: {e}"),
        },
    };
    let status_text_len = status_text.len() as u16;

    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
    let [table_area, status_bar_area] = layout.areas(area);
    let status_bar_layout = Layout::horizontal([
        Constraint::Max(keybinds_text_len + 10),
        Constraint::Max(status_text_len as u16),
    ]);
    let [keybinds_area, status_area] = status_bar_layout.areas(status_bar_area);
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
        Constraint::Length(25),
        Constraint::Length(50),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new())
        .header(Row::new(vec!["ID", "From", "Subject"]).style(Style::new().bold()))
        .row_highlight_style(Style::new().reversed());

    let keybinds = Paragraph::new(keybinds_text);
    let status = Paragraph::new(status_text).style(if status_text_len == 0 {
        Style::default()
    } else {
        Style::default().reversed()
    });

    StatefulWidget::render(table, table_area, buf, &mut *table_state);
    keybinds.render(keybinds_area, buf);
    status.render(status_area, buf);
}

fn render_message(app: &App, selected: usize, area: Rect, buf: &mut Buffer) {
    let message = app.get_message(selected);
    Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .render(area, buf);
}

fn render_compose(app: &App, focus: &ComposeFocus, area: Rect, buf: &mut Buffer) {
    let default_style = Style::default();
    let reversed_style = default_style.reversed();

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let [to_area, subject_area, message_area, keybind_area] = layout.areas(area);

    let to_layout = Layout::horizontal([Constraint::Length(9), Constraint::Max(71)]);
    let [to_label_area, to_input_area] = to_layout.areas(to_area);
    let subject_layout = Layout::horizontal([Constraint::Length(9), Constraint::Max(71)]);
    let [subject_label_area, subject_input_area] = subject_layout.areas(subject_area);

    let to_label = Line::from("To: ").right_aligned().style(match focus {
        ComposeFocus::To(ComposeMode::Normal) => reversed_style,
        _ => default_style, // hide cursor
    });
    let mut to_input = app.compose_to_input().borrow_mut();
    let subject_label = Line::from("Subject: ").right_aligned().style(match focus {
        ComposeFocus::Subject(ComposeMode::Normal) => reversed_style,
        _ => default_style, // hide cursor
    });
    let mut subject_input = app.compose_subject_input().borrow_mut();
    let mut message = app.compose_message_input().borrow_mut();
    let keybinds = match focus {
        ComposeFocus::Message(ComposeMode::Editing) => Line::from("Esc: Stop editing"),
        _ => Line::from("q: Back Tab: Next field Enter: Select field"),
    };

    to_input.set_cursor_line_style(default_style);
    to_input.set_cursor_style(match focus {
        ComposeFocus::To(ComposeMode::Editing) => reversed_style,
        _ => default_style, // hide cursor
    });
    subject_input.set_cursor_line_style(default_style);
    subject_input.set_cursor_style(match focus {
        ComposeFocus::Subject(ComposeMode::Editing) => reversed_style,
        _ => default_style, // hide cursor
    });
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

    to_label.render(to_label_area, buf);
    to_input.render(to_input_area, buf);
    subject_label.render(subject_label_area, buf);
    subject_input.render(subject_input_area, buf);
    message.render(message_area, buf);
    keybinds.render(keybind_area, buf);
}
