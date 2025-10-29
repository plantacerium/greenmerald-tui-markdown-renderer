use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use tui_markdown;
use crate::app::state::{PreviewState, App};
use crate::ui::widget::format_entry_flat;
use crate::ui::widget::format_entry_tree;

pub mod widget;


// ## 4. UI Drawing ##
// This function draws the entire UI on every frame
pub fn ui<B: Backend>(f: &mut Frame, app: &mut App) {
    // 1. Define main layout
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(1), Constraint::Min(0)])
    .split(f.area());

    let header_chunk = chunks[0];
    let content_chunk = chunks[1];

    // 2. Render Header
    let path_str = app.current_path.to_string_lossy();
    let header =
    Paragraph::new(path_str.as_ref()).style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(header, header_chunk);

    // 3. Define content layout
    let content_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(25),
                 Constraint::Percentage(40),
                 Constraint::Percentage(35),
    ])
    .split(content_chunk);

    // 4. Render Panel 1: Parent
    let parent_items: Vec<ListItem> = app
    .parent_entries
    .iter()
    .map(|entry| format_entry_flat(entry)) // Use flat formatter
    .collect();
    let parent_list = List::new(parent_items)
    .block(Block::default().borders(Borders::ALL).title("Parent"))
    .style(Style::default().fg(Color::DarkGray));
    f.render_stateful_widget(parent_list, content_chunks[0], &mut app.parent_selected);

    // 5. Render Panel 2: Current (The main "tree")
    // CHANGED: Title now shows view mode, items map calls format_entry_tree
    let title = if app.recursive_view {
        "Current (Recursive 'e')"
    } else {
        "Current (Flat 'e')"
    };
    let current_items: Vec<ListItem> = app
    .current_entries
    .iter()
    .map(|tree_entry| format_entry_tree(tree_entry)) // Use tree formatter
    .collect();
    let current_list = List::new(current_items)
    .block(Block::default().borders(Borders::ALL).title(title))
    .highlight_style(
        Style::default()
        .bg(Color::LightBlue)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD),
    );
    f.render_stateful_widget(current_list, content_chunks[1], &mut app.current_selected);

    // 6. Render Panel 3: Preview
    let preview_block = Block::default()
    .borders(Borders::ALL)
    .title("Preview (Ctrl+j/k to scroll)");
    let preview_chunk = content_chunks[2];

    match &app.preview_content {
        PreviewState::None => {
            f.render_widget(
                Paragraph::new("No item selected").block(preview_block),
                            preview_chunk,
            );
        }
        PreviewState::Text(text) => {
            let p = Paragraph::new(text.as_str())
            .block(preview_block)
            .wrap(ratatui::widgets::Wrap { trim: false })
            .scroll((app.preview_scroll_offset, 0)); // Use our u16 offset
            f.render_widget(p, preview_chunk);
        }
        PreviewState::Markdown(content) => {
            // Use tui_markdown to parse the string into ratatui::text::Text

            let text: Text<'_> = tui_markdown::from_str(content);
            // Render it as a normal, scrollable Paragraph
            let p = Paragraph::new(text)
            .block(preview_block)
            .wrap(ratatui::widgets::Wrap { trim: false })
            .scroll((app.preview_scroll_offset, 0));
            f.render_widget(p, preview_chunk);
        }
    }
}
