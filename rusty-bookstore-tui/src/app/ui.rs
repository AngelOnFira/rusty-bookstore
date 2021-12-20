use std::time::Duration;

use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table};
use tui::{symbols, Frame};
use tui_logger::TuiLoggerWidget;

use super::actions::Actions;
use super::state::AppState;
use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Min(10),   // Book table
                Constraint::Length(20),
            ]
            .as_ref(),
        )
        .split(size);

    // Title
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Body & Help
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    let body = draw_body(app.is_loading(), app.state());
    match app.state_mut() {
        AppState::Initialized {
            books_tablestate, ..
        } => rect.render_stateful_widget(body, body_chunks[0], books_tablestate),
        _ => rect.render_widget(body, body_chunks[0]),
    }

    let help = draw_help(app.actions());
    rect.render_widget(help, body_chunks[1]);

    // Logs
    let logs = draw_logs();
    rect.render_widget(logs, chunks[2]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Rusty Bookstore")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

fn draw_body<'a>(_loading: bool, state: &AppState) -> Table<'a> {
    let books = if let Some(books) = state.books() {
        let mut rows = Vec::new();
        for book in books {
            rows.push(Row::new(vec![
                Cell::from(book.name.to_string()),
                Cell::from(book.isbn.to_string()),
                Cell::from(book.price.to_string()),
            ]));
        }
        Table::new(rows).header(
            Row::new(vec![
                Cell::from("Title"),
                Cell::from("ISBN"),
                Cell::from("Price"),
            ])
            .style(Style::default().fg(Color::Yellow))
            // If you want some space between the header and the rest of the rows, you can always
            // specify some margin at the bottom.
            .bottom_margin(1),
        )
    } else {
        Table::new(vec![Row::new(vec![Cell::from("No books found")])])
    };

    return books
        // You can set the style of the entire Table.
        .style(Style::default().fg(Color::White))
        // It has an optional header, which is simply a Row always visible at the top.
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Books"))
        // Columns widths are constrained in the same way as Layout...
        .widths(&[
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Length(5),
        ])
        // ...and they can be separated by a fixed spacing.
        .column_spacing(4)
        // If you wish to highlight a row in any specific way when it is selected...
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
}

fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}
